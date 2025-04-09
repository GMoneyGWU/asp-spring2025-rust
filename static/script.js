//Adding a listener for the login form submission
document.getElementById('login-form')?.addEventListener('submit',async(e) => {
    e.preventDefault();
    //Grab the login details from the inputs
    const credentials={
        username:document.getElementById('username').value,
        password:document.getElementById('password').value
    };
    try{
        const response=await fetch('/api/login',{
            method:'POST',
            headers:{'Content-Type': 'application/json'},
            body:JSON.stringify(credentials)
        });
        const message=document.getElementById('message');
        const data=await response.json();
        if(response.ok){
            //Worked fine, so log it and redirect based on role
            console.log('Logged in, role is:',data.role);
            if(data.role==='admin')
                window.location.href='admin.html';
            else if(data.role==='student'){
                window.location.href='student.html';
            }
        }else{
            //Show the error if something’s off
            message.textContent=data.error;
            message.style.color='#ff4757';
        }
    }catch(err){
        //Catch any network or parsing issues
        console.error("Login failed with:",err);
        document.getElementById('message').textContent='Login didn’t work.Try again.';
    }
});

//Handles logging out when called
async function logout(){
    try{
        const response=await fetch('/api/logout',{
            method:'POST',
            headers:{'Content-Type':'application/json'},
            body:JSON.stringify({})
        });
        if(response.ok){
            //Back to the homepage if it works
            window.location.href='index.html';
        }else{
            const data=await response.json();
            alert(`Logout failed:${data.error}`);
        }
    }catch(err){
        //Something broke, let the user know
        console.error("Logout issue:",err);
        alert('Couldn’t log out.Try again.');
    }
}

//Sets up the admin course addition form
document.getElementById('add-course-form')?.addEventListener('submit',async(e) => {
    e.preventDefault();
    const course={
        dept_code:"CS", //Hardcoding this since it’s always CS
        course_number:document.getElementById('course-number').value,
        instructor:document.getElementById('instructor').value,
        description: document.getElementById('description').value,
        location:document.getElementById('location').value,
        meeting_time:getMeetingTimeString('day','start-hour','start-minute','end-hour','end-minute')
    };
    try{
        const response=await fetch('/api/admin/add_course',{
            method:'POST',
            headers:{'Content-Type':'application/json'},
            body:JSON.stringify(course)
        });
        const message=document.getElementById('message');
        const data=await response.json();
        if(response.ok){
            //Tell the admin it worked and reset the form
            message.textContent='Course added!';
            message.style.color='#00ff87';
            document.getElementById('add-course-form').reset();
            loadCoursesAdmin();
        }else{
            message.textContent=`Couldn’t add course:${data.error}`;
            message.style.color='#ff4757';
        }
    }catch(err){
        //Handle any fetch errors
        console.error("Error adding course:",err);
        document.getElementById('message').textContent='Adding course failed.Try again.';
    }
});

//Admin form for adding a new student user
document.getElementById('add-user-form')?.addEventListener('submit',async(e) => {
    e.preventDefault();
    const user={
        username:document.getElementById('new-username').value,
        password:document.getElementById('new-password').value,
        role:"student" //Always a student here
    };
    try{
        const response=await fetch('/api/admin/add_user',{
            method: 'POST',
            headers:{'Content-Type':'application/json'},
            body:JSON.stringify(user)
        });
        const message=document.getElementById('user-message');
        const data=await response.json();
        if(response.ok){
            message.textContent='Student added successfully!';
            message.style.color='#00ff87';
            document.getElementById('add-user-form').reset();
            loadUsersAdmin(); //Refresh the list
        }else{
            //Display the error if it fails
            message.textContent=`Failed to add:${data.error}`;
            message.style.color='#ff4757';
        }
    }catch(err){
        console.error("User add error:",err);
        document.getElementById('user-message').textContent='Couldn’t add student.Try again.';
    }
});

//Fetches and displays courses for the admin
async function loadCoursesAdmin(){
    try{
        const response=await fetch('/api/courses');
        if(!response.ok)throw new Error('Fetch failed');
        const courses=await response.json();
        const courseList=document.getElementById('course-list-admin');
        if(courseList){
            courseList.innerHTML=''; //Clear out old stuff
            const filters=getAdminFilters();
            const filteredCourses=filterCourses(courses,filters);
            filteredCourses.forEach(course => {
                const div=document.createElement('div');
                div.className='course';
                div.dataset.courseId=course.id;
                div.innerHTML=`
                    <strong>CS ${course.course_number}</strong> - ${course.instructor}<br>
                    ${course.description} (${course.location})<br>
                    Time:${course.meeting_time}
                    <button class="edit-button">Edit</button>
                    <button class="delete-button">Delete</button>
                `;
                courseList.appendChild(div);
            });
        }
    }catch(err){
        //Log any issues with loading courses
        console.error("Courses didn’t load:",err);
    }
}

//Pulls the filter values for admin course search
function getAdminFilters(){
    return{
        dept_code:document.getElementById('search-dept-admin')?.value.toLowerCase()||'',
        instructor:document.getElementById('search-instructor-admin')?.value.toLowerCase()|| '',
        description: document.getElementById('search-desc-admin')?.value.toLowerCase()||'',
        course_number:document.getElementById('search-number-admin')?.value.toLowerCase()||''
    };
}

//Loads the student list for admin view
async function loadUsersAdmin(){
    try{
        const response=await fetch('/api/admin/get_users');
        if(!response.ok)throw new Error('Users fetch failed');
        const users=await response.json();
        const userList=document.getElementById('user-list');
        if(userList){
            userList.innerHTML=''; //Start fresh
            users.forEach(user => {
                const div=document.createElement('div');
                div.className='course';
                div.dataset.username=user.username;
                div.innerHTML=`
                    <strong>${user.username}</strong><br>
                    Role:${user.role}
                    <button class="delete-user-button">Delete</button>
                `;
                userList.appendChild(div);
            });
        }
    }catch(err){
        console.error("Error loading users:",err);
    }
}

//Deletes a student user for admin
async function deleteUser(username){
    if(!confirm(`Really delete '${username}'?`))return;
    try{
        const response=await fetch('/api/admin/delete_user',{
            method:'POST',
            headers:{'Content-Type': 'application/json'},
            body:JSON.stringify(username)
        });
        const data=await response.json();
        if(response.ok){
            loadUsersAdmin(); //Update the list
        }else{
            alert(`Delete failed:${data.error}`);
        }
    }catch(err){
        console.error("Delete user error:",err);
        alert('Couldn’t delete student.Try again.');
    }
}

//Shows available courses for students
async function loadCoursesStudent(){
    try{
        const response=await fetch('/api/courses');
        if(!response.ok)throw new Error('Courses fetch bombed');
        const courses=await response.json();
        const courseList=document.getElementById('course-list');
        if(courseList){
            courseList.innerHTML=''; //Wipe it clean
            const filters=getStudentFilters();
            const filteredCourses=filterCourses(courses,filters);
            filteredCourses.forEach(course => {
                const div=document.createElement('div');
                div.className='course';
                div.dataset.courseId=course.id;
                div.innerHTML=`
                    <strong>CS ${course.course_number}</strong> - ${course.instructor}<br>
                    ${course.description} (${course.location})<br>
                    Time:${course.meeting_time}
                    <button class="add-button">Add to Schedule</button>
                `;
                courseList.appendChild(div);
            });
        }
    }catch(err){
        console.error("Student course load failed:",err);
    }
}

//Gets filter values for student course search
function getStudentFilters(){
    return{
        dept_code:document.getElementById('search-dept')?.value.toLowerCase()||'',
        instructor:document.getElementById('search-instructor')?.value.toLowerCase()||'',
        description: document.getElementById('search-desc')?.value.toLowerCase()|| '',
        course_number:document.getElementById('search-number')?.value.toLowerCase()||''
    };
}

//Filters courses based on what the user typed
function filterCourses(courses,filters){
    return courses.filter(course=>
        course.dept_code.toLowerCase().includes(filters.dept_code)&&
        course.instructor.toLowerCase().includes(filters.instructor)&&
        course.description.toLowerCase().includes(filters.description)&&
        course.course_number.toLowerCase().includes(filters.course_number)
    );
}

//Loads the student’s schedule into a calendar
async function loadSchedule(){
    const calendar=document.getElementById('calendar');
    if(!calendar){
        console.error("Can’t find calendar element");
        return;
    }

    console.log("Starting to load schedule...");
    calendar.innerHTML="<p>Loading...</p>";

    try{
        const response=await fetch('/api/student/schedule');
        console.log("Fetch status:",response.status);
        if(!response.ok){
            console.error("Fetch failed:",response.status);
            calendar.innerHTML=`<p>Error:${response.statusText} (Student login issue?)</p>`;
            return;
        }

        const schedule=await response.json();
        console.log("Got schedule:",JSON.stringify(schedule,null,2));

        //Build the calendar structure
        calendar.innerHTML=`
            <div class="calendar-time-column"></div>
            <div class="calendar-day" data-day="M"><div class="calendar-header">Monday</div></div>
            <div class="calendar-day" data-day="T"><div class="calendar-header">Tuesday</div></div>
            <div class="calendar-day" data-day="W"><div class="calendar-header">Wednesday</div></div>
            <div class="calendar-day" data-day="Th"><div class="calendar-header">Thursday</div></div>
            <div class="calendar-day" data-day="F"><div class="calendar-header">Friday</div></div>
        `;
        const hours=["08:00","09:00","10:00","11:00","12:00","13:00","14:00","15:00","16:00","17:00","18:00","19:00","20:00","21:00"];
        const timeColumn=calendar.querySelector('.calendar-time-column');
        hours.forEach(hour => {
            const timeDiv=document.createElement('div');
            timeDiv.className='calendar-time';
            timeDiv.textContent=hour;
            timeColumn.appendChild(timeDiv);
        });
        console.log("Calendar grid set up");

        if(!Array.isArray(schedule)||schedule.length===0){
            console.log("No scheduled courses");
            calendar.innerHTML+="<p>No courses yet.</p>";
            return;
        }

        const HEADER_HEIGHT=40; //Space for the headers

        schedule.forEach(([entry,course],index) => {
            console.log(`Processing entry ${index}:`,{entry,course});
            const slots=course.meeting_time.split(", ");
            slots.forEach((slot,slotIndex) => {
                console.log(`Slot ${slotIndex}:`,slot);

                const parts=slot.trim().split(' ');
                let days,time;
                if(parts.length===2){
                    [days,time]=parts;
                }else{
                    console.warn(`Bad slot format:${slot}`);
                    return;
                }

                const[start,end]=time.split('-');
                const startHour=parseInt(start.split(':')[0],10);
                const startMinute=parseInt(start.split(':')[1],10);
                const endHour=parseInt(end.split(':')[0],10);
                const endMinute=parseInt(end.split(':')[1],10);

                console.log(`Parsed:${days},start=${startHour}:${startMinute},end=${endHour}:${endMinute}`);

                const formatTime=(hour,minute)=>`${hour.toString().padStart(2,'0')}:${minute.toString().padStart(2,'0')}`;
                const startTime=formatTime(startHour,startMinute);
                const endTime=formatTime(endHour,endMinute);

                const dayMap={
                    'M':['M'],
                    'T':['T'],
                    'W':['W'],
                    'Th':['Th'],
                    'F':['F'],
                    'TTh':['T','Th'],
                    'MW':['M','W'],
                    'MWF':['M','W','F']
                };
                const resolvedDays=dayMap[days]||[days];

                resolvedDays.forEach(day => {
                    const startInMinutes=(startHour-8)*60+startMinute;
                    const endInMinutes=(endHour-8)*60+endMinute;
                    const durationInMinutes=endInMinutes-startInMinutes;

                    const dayColumn=calendar.querySelector(`.calendar-day[data-day="${day}"]`);
                    if(dayColumn){
                        const event=document.createElement('div');
                        event.className='calendar-event';
                        event.dataset.courseId=course.id;
                        event.textContent=`CS ${course.course_number} (${startTime}-${endTime})`;
                        event.style.position='absolute';
                        event.style.top=`${startInMinutes*0.833+HEADER_HEIGHT}px`;
                        event.style.height=`${durationInMinutes*0.833}px`;
                        event.style.left='0';
                        event.style.right='0';
                        dayColumn.appendChild(event);
                        console.log(`Added event for CS ${course.course_number} on ${day}`);
                    }else{
                        console.warn(`No column for ${day}`);
                    }
                });
            });
        });
    }catch(err){
        //Something went wrong, show it in the UI
        console.error("Schedule load error:",err);
        calendar.innerHTML="<p>Couldn’t load schedule.Check console.</p>";
    }
}

//Builds the meeting time string from form inputs
function getMeetingTimeString(dayPrefix,startHourPrefix,startMinutePrefix,endHourPrefix,endMinutePrefix){
    const days=document.getElementsByName(dayPrefix);
    const startHours=document.getElementsByName(startHourPrefix);
    const startMinutes=document.getElementsByName(startMinutePrefix);
    const endHours=document.getElementsByName(endHourPrefix);
    const endMinutes=document.getElementsByName(endMinutePrefix);
    
    let meetingTimes=[];
    for(let i=0;i<days.length;i++){
        if(days[i].checked){
            const startHour=startHours[i].value.padStart(2,'0');
            const endHour=endHours[i].value.padStart(2,'0');
            const time=`${days[i].value} ${startHour}:${startMinutes[i].value}-${endHour}:${endMinutes[i].value}`;
            meetingTimes.push(time);
        }
    }
    return meetingTimes.join(", ");
}

//Opens the edit popup for a course
async function editCourse(courseId){
    try{
        const response=await fetch('/api/courses');
        if(!response.ok)throw new Error('Fetch failed');
        const courses=await response.json();
        const course=courses.find(c=>c.id===courseId);
        if(!course){
            console.warn(`Course ID ${courseId} not found`);
            return;
        }

        const popup=document.getElementById('edit-popup');
        document.getElementById('edit-dept').value=course.dept_code;
        document.getElementById('edit-number').value=course.course_number;
        document.getElementById('edit-instructor').value=course.instructor;
        document.getElementById('edit-description').value=course.description;
        document.getElementById('edit-location').value=course.location;

        const days=document.getElementsByName('edit-day');
        const startHours=document.getElementsByName('edit-start-hour');
        const startMinutes=document.getElementsByName('edit-start-minute');
        const endHours=document.getElementsByName('edit-end-hour');
        const endMinutes=document.getElementsByName('edit-end-minute');

        const slots=course.meeting_time.split(", ");
        days.forEach(day=>day.checked=false);
        slots.forEach(slot => {
            const[day,time]=slot.trim().split(' ');
            const[start,end]=time.split('-');
            const startHour=start.split(':')[0];
            const startMinute=start.split(':')[1];
            const endHour=end.split(':')[0];
            const endMinute=end.split(':')[1];
            const index=['M','T','W','Th','F'].indexOf(day);
            if(index!==-1){
                days[index].checked=true;
                startHours[index].value=startHour;
                startMinutes[index].value=startMinute;
                endHours[index].value=endHour;
                endMinutes[index].value=endMinute;
            }
        });

        const editMessage=document.getElementById('edit-message');
        editMessage.textContent="";
        popup.style.display='flex';

        document.getElementById('edit-course-form').onsubmit=async(e) => {
            e.preventDefault();
            const meetingTime=getMeetingTimeString('edit-day','edit-start-hour','edit-start-minute','end-hour','end-minute');
            if(!meetingTime){
                alert("Need at least one day and time!");
                editMessage.textContent="Pick a day and time.";
                return;
            }

            const newData={
                id:courseId,
                dept_code:document.getElementById('edit-dept').value,
                course_number:document.getElementById('edit-number').value,
                instructor:document.getElementById('edit-instructor').value,
                description: document.getElementById('edit-description').value,
                location:document.getElementById('edit-location').value,
                meeting_time:meetingTime
            };
            try{
                const response=await fetch('/api/admin/update_course',{
                    method:'POST',
                    headers:{'Content-Type':'application/json'},
                    body:JSON.stringify(newData)
                });
                const data=await response.json();
                if(response.ok){
                    popup.style.display='none';
                    loadCoursesAdmin();
                }else{
                    alert(`Update failed:${data.error}`);
                }
            }catch(err){
                console.error("Update error:",err);
                alert('Couldn’t update course.Try again.');
            }
        };

        document.getElementById('cancel-button').onclick=() => {
            popup.style.display='none';
        };
    }catch(err){
        console.error("Edit course error:",err);
    }
}

//Deletes a course for admin
async function deleteCourse(courseId){
    if(!confirm("Sure you want to delete this course?"))return;
    try{
        const response=await fetch('/api/admin/delete_course',{
            method:'POST',
            headers:{'Content-Type': 'application/json'},
            body:JSON.stringify(courseId)
        });
        const data=await response.json();
        if(response.ok){
            document.getElementById('edit-popup').style.display='none';
            loadCoursesAdmin();
        }else{
            alert(`Delete failed:${data.error}`);
        }
    }catch(err){
        console.error("Delete course error:",err);
        alert('Error deleting course.Try again.');
    }
}

//Adds a course to the student’s schedule
async function addToSchedule(courseId){
    try{
        const response=await fetch('/api/student/add_to_schedule',{
            method:'POST',
            headers:{'Content-Type':'application/json'},
            body:JSON.stringify(courseId)
        });
        const data=await response.json();
        if(response.ok){
            loadSchedule(); //Refresh the calendar
        }else{
            alert(`Couldn’t add to schedule:${data.error}`);
        }
    }catch(err){
        console.error('Add to schedule failed:',err);
        alert('Error adding course.Try again.');
    }
}

//Edits notes or slot for a scheduled course
async function editScheduleEntry(courseId){
    try{
        const response=await fetch('/api/student/schedule');
        if(!response.ok)throw new Error('Schedule fetch failed');
        const schedule=await response.json();
        const entry=schedule.find(([sc])=>sc.course_id===courseId);
        if(!entry)return;

        const[scheduledCourse]=entry;
        const popup=document.getElementById('notes-popup');
        popup.dataset.courseId=courseId;
        document.getElementById('notes-text').value=scheduledCourse.notes;
        document.getElementById('notes-slot').value=scheduledCourse.slot;
        popup.style.display='flex';

        document.getElementById('notes-form').onsubmit=async(e) => {
            e.preventDefault();
            const updatedEntry={
                course_id:courseId,
                notes:document.getElementById('notes-text').value,
                slot:document.getElementById('notes-slot').value
            };
            const response=await fetch('/api/student/update_schedule_entry',{
                method:'POST',
                headers:{'Content-Type':'application/json'},
                body:JSON.stringify(updatedEntry)
            });
            if(response.ok){
                popup.style.display='none';
                loadSchedule();
            }else{
                const data=await response.json();
                alert(`Couldn’t save notes:${data.error}`);
            }
        };
    }catch(err){
        console.error("Edit schedule error:",err);
    }
}

//Drops a course from the student’s schedule
async function dropFromSchedule(courseId){
    if(!confirm("Really drop this course?"))return;
    console.log('Trying to drop course ID:',courseId);
    try{
        const response=await fetch('/api/student/drop_from_schedule',{
            method:'POST',
            headers:{'Content-Type': 'application/json'},
            body:JSON.stringify(courseId)
        });
        const data=await response.json();
        console.log('Drop response:',response.status,data);
        if(response.ok){
            document.getElementById('notes-popup').style.display='none';
            loadSchedule();
            console.log('Dropped course:',courseId);
        }else{
            alert(`Drop failed:${data.error}`);
            console.error('Drop error:',data.error);
        }
    }catch(err){
        console.error('Drop failed:',err);
        alert('Couldn’t drop course.Try again.');
    }
}

//Sets up tab switching behavior
function setupTabs(){
    document.querySelectorAll('.tab-button').forEach(button => {
        button.addEventListener('click',() => {
            document.querySelectorAll('.tab-button').forEach(btn=>btn.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(content=>content.classList.remove('active'));
            button.classList.add('active');
            document.getElementById(button.dataset.tab).classList.add('active');
            if(button.dataset.tab==='manage-user'){
                loadUsersAdmin();
            }else if(button.dataset.tab==='schedule'){
                loadSchedule();
            }
        });
    });
}

//Adds listeners for search inputs
function setupSearchListeners(isAdmin){
    const prefix=isAdmin?'-admin':'';
    ['dept','instructor','desc','number'].forEach(field => {
        document.getElementById(`search-${field}${prefix}`)?.addEventListener('input',() => {
            isAdmin?loadCoursesAdmin():loadCoursesStudent();
        });
    });
}

//Handles clicks with event delegation
document.addEventListener('click',(e) => {
    const courseId=parseInt(e.target.closest('#notes-popup')?.dataset.courseId||e.target.parentElement?.dataset.courseId,10);
    if(e.target.classList.contains('drop-button')){
        if(!courseId){
            console.error("No courseId for drop");
            alert("Error:No course ID.");
            return;
        }
        dropFromSchedule(courseId);
    }
    const username=e.target.parentElement?.dataset.username;
    
    if(e.target.classList.contains('add-button')){
        addToSchedule(courseId);
    }else if(e.target.classList.contains('edit-button')){
        editCourse(courseId);
    }else if(e.target.classList.contains('delete-button')){
        deleteCourse(courseId);
    }else if(e.target.classList.contains('delete-user-button')){
        deleteUser(username);
    }else if(e.target.id==='logout-button'){
        logout();
    }else if(e.target.classList.contains('calendar-event')){
        editScheduleEntry(parseInt(e.target.dataset.courseId,10));
    }
});

//Closes popups when clicking outside
document.getElementById('edit-popup')?.addEventListener('click',(e) => {
    if(e.target===e.currentTarget){
        e.currentTarget.style.display='none';
    }
});
document.getElementById('notes-popup')?.addEventListener('click',(e) => {
    if(e.target===e.currentTarget)
        e.currentTarget.style.display='none';
});

//Runs initial setup based on page
if(document.getElementById('course-list-admin')){
    loadCoursesAdmin();
    setupSearchListeners(true);
    setupTabs();
}else if(document.getElementById('course-list')){
    loadCoursesStudent();
    loadSchedule();
    setupSearchListeners(false);
    setupTabs();
}