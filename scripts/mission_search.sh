echo "Test-Case 1: Mission Search: "
curl http://localhost:6789/search_mission\
 -H "Content-Type:application/json"\
 -d '{"keyword":"Shit"}'
echo ""
