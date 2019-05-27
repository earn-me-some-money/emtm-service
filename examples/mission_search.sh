echo "Test-Case 1: Student Receive Task: "
curl http://localhost:8088/search_mission\
 -H "Content-Type:application/json"\
 -d '{"keyword":"Shit"}'
echo ""