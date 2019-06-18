echo "Get Cow Info Test Case 1: (True)"
curl http://localhost:6789/get_cow_info\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12306"}'
echo ""

echo "Get Cow Info Test Case 2: (False)"
curl http://localhost:6789/get_cow_info\
 -H "Content-Type:application/json"\
 -d '{"userid":"wechat12307"}'
echo ""

