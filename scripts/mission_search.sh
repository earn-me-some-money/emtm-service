echo "Test-Case 1: Mission Search: "
curl http://localhost:6789/task/search\
 -H "Content-Type:application/json"\
 -X GET\
 -d '{"keyword":"Shit"}'
echo ""
