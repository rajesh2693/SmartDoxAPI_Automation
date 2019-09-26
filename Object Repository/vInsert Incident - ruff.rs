<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>vInsert Incident - ruff</name>
   <tag></tag>
   <elementGuidId>4b7e29d3-c652-430d-928f-b2e7feb56ea0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;incidentId\&quot;:\&quot;75F96CE9B5BE4CC7A510E5E4D7FFE7F4\&quot;,\n  \&quot;incidentDate\&quot;:\&quot;04-07-2019 22:21:08 +05:30\&quot;,\n   \&quot;diseaseID\&quot;:\&quot;ED8A1851-0DF6-498C-BAA8-A6747043790E\&quot;,\n    \&quot;plantAgeInDays\&quot;:2,\n    \&quot;geographyL1Id\&quot;:\&quot;F8208FF2-1E05-4292-BF8C-6804C98E22CA\&quot;,\n    \&quot;geographyL2Id\&quot;:\&quot;8839FE82-D4B5-4A24-A70E-6600329583E2\&quot;,\n    \&quot;geographyL3Id\&quot;:\&quot;C93673E3-7446-4AFA-8979-052E1E841E66\&quot;,\n  \&quot;checkInLocality\&quot;:\&quot;bangalore\&quot;,\n  \&quot;locationLatitude\&quot; :12.89898,\n  \&quot;locationLongitude\&quot;:12.089898,\n   \&quot;incidentLocality\&quot;:\&quot;Bangalore\&quot;,\n    \&quot;incidentSeverity\&quot;:\&quot;Low\&quot;\n    \n \n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tenantId</name>
      <type>Main</type>
      <value>b68f64ef-0f8f-47a9-b192-ad8e9e6a5d20</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>userId</name>
      <type>Main</type>
      <value>1b6bb97e-4935-442c-b85d-0004f4db21ef</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.130.1.49:2022/v1/api/insert-incident?uniqueToken=f1c1fa29-9d4a-4693-9779-a2b837746f84</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()





WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)



WS.verifyElementPropertyValue(response, 'incidentId', &quot;DRP040720191741133&quot;)
WS.verifyElementPropertyValue(response, 'message', &quot;Success&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
