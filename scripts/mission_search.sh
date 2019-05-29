echo "Test-Case 1: Mission Search: "
curl http://localhost:8088/search_mission\
 -H "Content-Type:application/json"\
 -d '{"keyword":"Shit"}'
echo ""