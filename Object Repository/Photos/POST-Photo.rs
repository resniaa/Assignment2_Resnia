<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST-Photo</name>
   <tag></tag>
   <elementGuidId>c4274b1d-739b-4678-a945-60b966d73d45</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;albumId\&quot;: 1,\n    \&quot;id\&quot;: 1,\n    \&quot;title\&quot;: \&quot;hijau\&quot;,\n    \&quot;url\&quot;: \&quot;https://via.placeholder.com/600/92c952\&quot;,\n    \&quot;thumbnailUrl\&quot;: \&quot;https://via.placeholder.com/150/92c952\&quot;\n }&quot;,
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
      <webElementGuid>7e5f9f1a-d77d-4001-9ca1-bdb95fb7f16f</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/photos</restUrl>
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


WS.verifyElementPropertyValue(response, 'albumId', '1')
WS.verifyElementPropertyValue(response, 'id', '5001')
WS.verifyElementPropertyValue(response, 'title', 'hijau')
WS.verifyElementPropertyValue(response, 'url', 'https://via.placeholder.com/600/92c952')
WS.verifyElementPropertyValue(response, 'thumbnailUrl', 'https://via.placeholder.com/150/92c952')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
