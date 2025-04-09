// Global variables
let courses = [];
let students = [];
let preferences = {};
let schedules = [];
let currentUser = 'G11045678'; // Hardcoded user ID for demo purposes

// API URL (backend)
const API_URL = 'http://localhost:8080';

// DOM Elements
const navCourses = document.getElementById('nav-courses');
const navSchedule = document.getElementById('nav-schedule');
const navPreferences = document.getElementById('nav-preferences');
const coursesList = document.getElementById('courses-list');
const courseSearch = document.getElementById('course-search');
const filterDepartment = document.getElementById('filter-department');
const scheduleGrid = document.getElementById('schedule-grid');
const scheduleSummary = document.getElementById('schedule-summary');
const generateScheduleBtn = document.getElementById('generate-schedule');
const semesterSelect = document.getElementById('semester-select');
const maxCredits = document.getElementById('max-credits');
const preferredCoursesSelection = document.getElementById('preferred-courses-selection');
const timeGridBody = document.getElementById('time-grid-body');
const savePreferencesBtn = document.getElementById('save-preferences');

// Event Listeners
document.addEventListener('DOMContentLoaded', initApp);
navCourses.addEventListener('click', showCoursesSection);
navSchedule.addEventListener('click', showScheduleSection);
navPreferences.addEventListener('click', showPreferencesSection);
courseSearch.addEventListener('input', filterCourses);
filterDepartment.addEventListener('change', filterCourses);
generateScheduleBtn.addEventListener('click', generateSchedule);
savePreferencesBtn.addEventListener('click', savePreferences);

// Initialize the application
async function initApp() {
    try {
        // Load data from backend
        courses = await fetchCourses();
        // For demo purposes, we'll use local data instead of API calls
        displayCourses(courses);
        createTimeGrid();
        loadUserPreferences();
        displaySchedule();
    } catch (error) {
        console.error('Error initializing app:', error);
        // For demo, load static data if API fails
        loadStaticData();
    }
}

// Fetch courses from API
async function fetchCourses() {
    try {
        const response = await fetch(`${API_URL}/courses`);
        if (!response.ok) {
            throw new Error('Failed to fetch courses');
        }
        return await response.json();
    } catch (error) {
        console.error('Error fetching courses:', error);
        // Fallback to static data for demo
        return loadStaticCourses();
    }
}

// Load static course data (fallback)
function loadStaticCourses() {
    // Simulating course data in case API is not available
    return [
        {
            id: "CSCI6221",
            name: "Advanced Software Paradigms",
            credits: 3,
            description: "Advanced programming language paradigms including functional, object-oriented, and logical. Formal principles of language design and implementation.",
            prerequisites: [],
            available_times: [
                {
                    day: "Monday",
                    start_time: "18:10",
                    end_time: "20:40"
                },
                {
                    day: "Wednesday",
                    start_time: "15:30",
                    end_time: "18:00"
                }
            ]
        },
        {
            id: "CSCI6212",
            name: "Algorithms",
            credits: 3,
            description: "Design and analysis of algorithms. Topics include sorting, selection, dynamic programming, graph algorithms, and NP-completeness.",
            prerequisites: [],
            available_times: [
                {
                    day: "Tuesday",
                    start_time: "15:30",
                    end_time: "18:00"
                },
                {
                    day: "Thursday",
                    start_time: "12:45",
                    end_time: "15:15"
                }
            ]
        },
        // Add a few more courses for the static demo
        {
            id: "CSCI6511",
            name: "Artificial Intelligence",
            credits: 3,
            description: "Theory and applications of artificial intelligence including knowledge representation, inference, machine learning, and planning.",
            prerequisites: ["CSCI6212"],
            available_times: [
                {
                    day: "Tuesday",
                    start_time: "12:45",
                    end_time: "15:15"
                },
                {
                    day: "Thursday",
                    start_time: "18:10",
                    end_time: "20:40"
                }
            ]
        }
    ];
}

// Display courses in the UI
function displayCourses(courseList) {
    coursesList.innerHTML = '';
    
    courseList.forEach(course => {
        const courseCard = document.createElement('div');
        courseCard.className = 'course-card';
        
        // Format available times for display
        const timeSlots = course.available_times.map(time => 
            `${time.day} ${time.start_time} - ${time.end_time}`
        ).join('<br>');
        
        courseCard.innerHTML = `
            <div class="course-id">${course.id}</div>
            <div class="course-title">${course.name}</div>
            <div class="course-credits">${course.credits} Credits</div>
            <div class="course-description">${course.description}</div>
            <div class="course-times">
                <strong>Available Times:</strong><br>
                ${timeSlots}
            </div>
            <button class="add-course-btn" data-id="${course.id}">Add to Schedule</button>
        `;
        
        // Add event listener for the add button
        const addButton = courseCard.querySelector('.add-course-btn');
        addButton.addEventListener('click', () => addCourseToSchedule(course));
        
        coursesList.appendChild(courseCard);
    });
}

// Filter courses based on search and filter criteria
function filterCourses() {
    const searchTerm = courseSearch.value.toLowerCase();
    const department = filterDepartment.value;
    
    const filteredCourses = courses.filter(course => {
        // Filter by search term
        const matchesSearch = 
            course.id.toLowerCase().includes(searchTerm) ||
            course.name.toLowerCase().includes(searchTerm) ||
            course.description.toLowerCase().includes(searchTerm);
        
        // Filter by department
        const matchesDepartment = !department || course.id.startsWith(department);
        
        return matchesSearch && matchesDepartment;
    });
    
    displayCourses(filteredCourses);
}

// Add a course to the schedule
function addCourseToSchedule(course) {
    // This would normally update the backend, for demo just show an alert
    alert(`Added ${course.name} to your schedule!`);
    
    // Redisplay schedule
    displaySchedule();
}

// Generate a schedule
function generateSchedule() {
    // This would call the backend algorithm to generate an optimal schedule
    alert('Schedule generation algorithm would run here');
    
    // For demo, just refresh the display
    displaySchedule();
}

// Display schedule in the grid
function displaySchedule() {
    // Create schedule grid (for demo, showing a basic weekly calendar)
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
    const timeSlots = ['9:00 AM', '11:00 AM', '1:00 PM', '3:00 PM', '5:00 PM', '7:00 PM'];
    
    let gridHTML = '<table id="schedule-grid">';
    
    // Headers
    gridHTML += '<tr><th class="time-header"></th>';
    days.forEach(day => {
        gridHTML += `<th class="day-header">${day}</th>`;
    });
    gridHTML += '</tr>';
    
    // Time slots
    timeSlots.forEach(time => {
        gridHTML += `<tr><td class="time-header">${time}</td>`;
        
        days.forEach(day => {
            // Check if there's a scheduled course at this day/time
            // For demo, simulate a simple schedule
            let courseHtml = '';
            
            if (day === 'Monday' && time === '5:00 PM') {
                courseHtml = `<div class="scheduled-course">
                    CSCI6221<br>Advanced Software Paradigms<br>SEH 1400
                </div>`;
            } else if (day === 'Tuesday' && time === '3:00 PM') {
                courseHtml = `<div class="scheduled-course">
                    CSCI6212<br>Algorithms<br>SEH 1500
                </div>`;
            }
            
            gridHTML += `<td class="time-slot">${courseHtml}</td>`;
        });
        
        gridHTML += '</tr>';
    });
    
    gridHTML += '</table>';
    
    document.querySelector('.schedule-grid-container').innerHTML = gridHTML;
    
    // Update summary
    scheduleSummary.innerHTML = `
        <h3>Schedule Summary</h3>
        <p>Total Credits: 6</p>
        <p>Courses: 2</p>
    `;
}

// Create the time grid for preferences
function createTimeGrid() {
    const timeSlots = ['9:00 AM', '11:00 AM', '1:00 PM', '3:00 PM', '5:00 PM', '7:00 PM'];
    const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday'];
    
    // Clear existing grid
    timeGridBody.innerHTML = '';
    
    // Add time labels
    timeSlots.forEach(time => {
        timeGridBody.innerHTML += `<div class="time-cell">${time}</div>`;
        
        // Add day cells for this time
        days.forEach(day => {
            const timeBlock = document.createElement('div');
            timeBlock.className = 'time-block';
            timeBlock.dataset.time = time;
            timeBlock.dataset.day = day;
            timeBlock.textContent = ' ';
            
            // Toggle availability on click
            timeBlock.addEventListener('click', () => {
                timeBlock.classList.toggle('unavailable');
            });
            
            timeGridBody.appendChild(timeBlock);
        });
    });
}

// Load user preferences (demo)
function loadUserPreferences() {
    // For demo, simulate loading user preferences
    maxCredits.value = 9;
    
    // Highlight some preferred courses (demo)
    setTimeout(() => {
        displayPreferredCourses();
    }, 500);
    
    // Mark some time slots as unavailable (demo)
    setTimeout(() => {
        const timeBlocks = document.querySelectorAll('.time-block');
        timeBlocks.forEach(block => {
            if (
                (block.dataset.day === 'Friday') ||
                (block.dataset.day === 'Monday' && block.dataset.time === '9:00 AM')
            ) {
                block.classList.add('unavailable');
            }
        });
    }, 500);
}

// Display preferred courses selection
function displayPreferredCourses() {
    // Create checkboxes for all courses
    preferredCoursesSelection.innerHTML = '';
    
    courses.forEach(course => {
        const label = document.createElement('label');
        label.className = 'checkbox-label';
        
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.value = course.id;
        
        // Simulate preferred courses for demo
        if (['CSCI6221', 'CSCI6212', 'CSCI6511'].includes(course.id)) {
            checkbox.checked = true;
        }
        
        label.appendChild(checkbox);
        label.appendChild(document.createTextNode(` ${course.id} - ${course.name}`));
        
        preferredCoursesSelection.appendChild(label);
        preferredCoursesSelection.appendChild(document.createElement('br'));
    });
}

// Save preferences
function savePreferences(e) {
    e.preventDefault();
    
    // Gather preferences data
    const max = maxCredits.value;
    
    const preferredCourses = Array.from(
        preferredCoursesSelection.querySelectorAll('input[type="checkbox"]:checked')
    ).map(checkbox => checkbox.value);
    
    const unavailableTimes = Array.from(
        document.querySelectorAll('.time-block.unavailable')
    ).map(block => ({
        day: block.dataset.day,
        time: block.dataset.time
    }));
    
    // Create preferences object
    const preferencesData = {
        student_id: currentUser,
        preferred_courses: preferredCourses,
        max_credits: parseInt(max),
        unavailable_times: unavailableTimes
    };
    
    // For demo, just show what would be sent to the backend
    console.log('Saving preferences:', preferencesData);
    alert('Preferences saved!');
    
    // In real implementation, this would be sent to the backend
    // savePreferencesToAPI(preferencesData);
}

// Navigation functions
function showCoursesSection() {
    document.getElementById('courses-section').className = 'section-visible';
    document.getElementById('schedule-section').className = 'section-hidden';
    document.getElementById('preferences-section').className = 'section-hidden';
    
    navCourses.classList.add('active');
    navSchedule.classList.remove('active');
    navPreferences.classList.remove('active');
}

function showScheduleSection() {
    document.getElementById('courses-section').className = 'section-hidden';
    document.getElementById('schedule-section').className = 'section-visible';
    document.getElementById('preferences-section').className = 'section-hidden';
    
    navCourses.classList.remove('active');
    navSchedule.classList.add('active');
    navPreferences.classList.remove('active');
    
    // Refresh schedule display
    displaySchedule();
}

function showPreferencesSection() {
    document.getElementById('courses-section').className = 'section-hidden';
    document.getElementById('schedule-section').className = 'section-hidden';
    document.getElementById('preferences-section').className = 'section-visible';
    
    navCourses.classList.remove('active');
    navSchedule.classList.remove('active');
    navPreferences.classList.add('active');
}

// Load static data for demo
function loadStaticData() {
    courses = loadStaticCourses();
    displayCourses(courses);
    createTimeGrid();
    loadUserPreferences();
    displaySchedule();
}