<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-User</name>
   <tag></tag>
   <elementGuidId>85af6b90-1aa9-424a-9b73-99e7da634f34</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: 1,\n    \&quot;name\&quot;: \&quot;Resnia Trya Muslima\&quot;,\n    \&quot;username\&quot;: \&quot;Resnia\&quot;,\n    \&quot;email\&quot;: \&quot;resnia@gmail.com\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;Jalan NN\&quot;,\n      \&quot;suite\&quot;: \&quot;16\&quot;,\n      \&quot;city\&quot;: \&quot;Tangerang Selatan\&quot;,\n      \&quot;zipcode\&quot;: \&quot;15416\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;-37.3159\&quot;,\n        \&quot;lng\&quot;: \&quot;81.1496\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;085714746591\&quot;,\n    \&quot;website\&quot;: \&quot;nianiania.org\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;Nia Company\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;Multi-layered client-server neural-net\&quot;,\n      \&quot;bs\&quot;: \&quot;harness real-time e-markets\&quot;\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a11129ab-4eca-4fe1-92d3-0c69955b250f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/todos</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 201)
assertThat(response.getStatusCode()).isEqualTo(201)


WS.verifyElementPropertyValue(response, 'id', '201')
WS.verifyElementPropertyValue(response, 'name', 'Resnia Trya Muslima')
WS.verifyElementPropertyValue(response, 'username', 'Resnia')
WS.verifyElementPropertyValue(response, 'email', 'resnia@gmail.com')
WS.verifyElementPropertyValue(response, 'address.street', 'Jalan NN')
WS.verifyElementPropertyValue(response, 'address.suite', '16')
WS.verifyElementPropertyValue(response, 'address.city', 'Tangerang Selatan')
WS.verifyElementPropertyValue(response, 'address.zipcode', '15416')
WS.verifyElementPropertyValue(response, 'address.geo.lat', '-37.3159')
WS.verifyElementPropertyValue(response, 'address.geo.lng', '81.1496')
WS.verifyElementPropertyValue(response, 'phone', '085714746591')
WS.verifyElementPropertyValue(response, 'website', 'nianiania.org')
WS.verifyElementPropertyValue(response, 'company.name', 'Nia Company')
WS.verifyElementPropertyValue(response, 'company.catchPhrase', 'Multi-layered client-server neural-net')
WS.verifyElementPropertyValue(response, 'company.bs', 'harness real-time e-markets')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
