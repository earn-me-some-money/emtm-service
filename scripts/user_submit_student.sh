echo "Test Case 1: (True):"
curl http://localhost:6789/task/submit-stu\
 -H "Content-Type:application/json"\
 -d '{"task_mid":1, "userid":"wechat12307", "poster_id":1, "answers":[{"order":0, "answer":"XiaoMIng"}, {"order":1, "choices":[0]}, {"order":2, "choices":[0,1,2]}]}'
echo ""

echo "Test Case 2: (False, mission type is not questionnaire):"
curl http://localhost:6789/task/submit-stu\
 -H "Content-Type:application/json"\
 -d '{"task_mid":2, "userid":"wechat12307", "poster_id":2, "answers":[{"order":0, "answer":"XiaoMIng"}, {"order":1, "choices":[0]}, {"order":2, "choices":[0,1,2]}]}'
echo ""

echo "Test Case 3: (False, current student user is not mission participant):"
curl http://localhost:6789/task/submit-stu\
 -H "Content-Type:application/json"\
 -d '{"task_mid":1, "userid":"wechat12308", "poster_id":2, "answers":[{"order":0, "answer":"XiaoMIng"}, {"order":1, "choices":[0]}, {"order":2, "choices":[0,1,2]}]}'
echo ""

echo "Test Case 4: (False, mission and poster not match):"
curl http://localhost:6789/task/submit-stu\
 -H "Content-Type:application/json"\
 -d '{"task_mid":1, "userid":"wechat12307", "poster_id":3, "answers":[{"order":0, "answer":"XiaoMIng"}, {"order":1, "choices":[0]}, {"order":2, "choices":[0,1,2]}]}'
echo ""
