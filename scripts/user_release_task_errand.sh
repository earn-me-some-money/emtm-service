echo "Test Case 1: Cow-User Release errand task: (False, cow user cannot release errand)"
curl http://localhost:6789/task/release-errand\
 -H "Content-Type:application/json"\
 -d '{"mid":1, "pickup_address":"SYSU", "deliver_address":"SYSU", "phone_number":"123", "pick_number":"086", "info":"Test"}'
echo ""

echo "Test Case 2: Student-User Release errand task: (True)"
curl http://localhost:6789/task/release-errand\
 -H "Content-Type:application/json"\
 -d '{"mid":2, "pickup_address":"SYSU", "deliver_address":"SYSU", "phone_number":"123", "pick_number":"086", "info":"Test"}'
echo ""

echo "Test Case 3: Student-User Release errand task Duplication: (False, Cannot rewrite task!)"
curl http://localhost:6789/task/release-errand\
 -H "Content-Type:application/json"\
 -d '{"mid":2, "pickup_address":"SYSU", "deliver_address":"SYSU", "phone_number":"123", "pick_number":"086", "info":"Duplication"}'
echo ""
