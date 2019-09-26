<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>gCrop-Master</name>
   <tag></tag>
   <elementGuidId>5b2a355b-2e0b-45df-ad54-e59a8af112f3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;versionTableId\&quot; : \&quot;${versiontableId}\&quot;,\n  \&quot;syncType\&quot; : \&quot;f\&quot;,\n \n  \n}&quot;,
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
   <restUrl>http://10.130.1.49:2022/v1/api/crop-master?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>0911a5a0-4842-4ec3-a6dc-fa9207cbcdb7</id>
      <masked>false</masked>
      <name>versiontableId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>56f53534-12ff-47fd-a787-4e5c464d527b</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>b8cfa8f6-ad42-43d6-82b6-2aaf6a2ad833</id>
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


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'cropInfo[0].cropName', &quot;Corn&quot;)
WS.verifyElementPropertyValue(response, 'cropInfo[0].cropId', &quot;3fae2c89-2173-4bda-a49b-1c27b5fc5e07&quot;)

WS.verifyElementPropertyValue(response, 'cropInfo[1].cropName', &quot;Vegetables&quot;)
WS.verifyElementPropertyValue(response, 'cropInfo[1].cropId', &quot;7a9cc341-d6d3-4858-91de-27f0c1969218&quot;)

WS.verifyElementPropertyValue(response, 'cropInfo[2].cropName', &quot;Plantation&quot;)
WS.verifyElementPropertyValue(response, 'cropInfo[2].cropId', &quot;b43f3a40-78a4-4f63-b0a3-e0582b21847d&quot;)

WS.verifyElementPropertyValue(response, 'cropInfo[3].cropName', &quot;Rice&quot;)
WS.verifyElementPropertyValue(response, 'cropInfo[3].cropId', &quot;a49318df-a1aa-41bf-8114-ff0b702a403c&quot;)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
