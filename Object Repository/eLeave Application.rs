<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>eLeave Application</name>
   <tag></tag>
   <elementGuidId>43454795-fbc9-4f8c-afdd-145135d32316</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;leaveTypeId\&quot;:${leavetypeId},\n  \&quot;fromDate\&quot;:\&quot;${fromdaTe}\&quot;,\n  \&quot;toDate\&quot;:\&quot;${todaTe}\&quot;,\n  \&quot;comments\&quot;:\&quot;${commenTs}\&quot;\n}\n\n&quot;,
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
      <value>${userId}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.130.1.49:2022/v1/api/Leave-Application?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b0625ef9-2ea1-4ead-8157-177bacc92da0</id>
      <masked>false</masked>
      <name>leavetypeId</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6b83b8b3-1c56-489a-ae7c-995c2f733e50</id>
      <masked>false</masked>
      <name>fromdaTe</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>70f9112d-b68d-4c27-90c4-b1408321ff19</id>
      <masked>false</masked>
      <name>todaTe</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>06b37824-c761-49af-af01-d12640ed0269</id>
      <masked>false</masked>
      <name>commenTs</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>c3fe9c9b-11f9-4adf-bc53-99b28c612b99</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>715dd243-c88d-45e2-9b4d-43a208955e6a</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyElementPropertyValue(response, 'message', &quot;Leave Applied Successfully&quot;)


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
