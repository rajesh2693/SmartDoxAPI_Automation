<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>uInsert Incident</name>
   <tag></tag>
   <elementGuidId>50bf256f-771d-4160-b31f-e3da980b3f6e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;incidentId\&quot;:\&quot;${incidentID}\&quot;,\n  \&quot;incidentDate\&quot;:\&quot;incidentDate\&quot;,\n   \&quot;diseaseID\&quot;:\&quot;ED8A1851-0DF6-498C-BAA8-A6747043790E\&quot;,\n    \&quot;plantAgeInDays\&quot;:2,\n    \&quot;geographyL1Id\&quot;:\&quot;F8208FF2-1E05-4292-BF8C-6804C98E22CA\&quot;,\n    \&quot;geographyL2Id\&quot;:\&quot;8839FE82-D4B5-4A24-A70E-6600329583E2\&quot;,\n    \&quot;geographyL3Id\&quot;:\&quot;C93673E3-7446-4AFA-8979-052E1E841E66\&quot;,\n  \&quot;checkInLocality\&quot;:\&quot;${checkInLocality}\&quot;,\n  \&quot;locationLatitude\&quot; :12.89898,\n  \&quot;locationLongitude\&quot;:12.089898,\n   \&quot;incidentLocality\&quot;:\&quot;${incidentLocality}\&quot;,\n    \&quot;incidentSeverity\&quot;:\&quot;Low\&quot;\n    \n \n}&quot;,
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
   <restUrl>http://10.130.1.49:2022/v1/api/insert-incident?uniqueToken=${uniqueToken}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.userid</defaultValue>
      <description></description>
      <id>ca853951-891a-41ef-a30c-d9deef92ec76</id>
      <masked>false</masked>
      <name>userId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.uniquetoken</defaultValue>
      <description></description>
      <id>488759e1-27b3-4b0f-b8ed-b94a272ecee2</id>
      <masked>false</masked>
      <name>uniqueToken</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9e15c074-1818-4245-bb28-8481d997585b</id>
      <masked>false</masked>
      <name>incidentID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e434600f-f4b1-465e-ac94-b5815742927b</id>
      <masked>false</masked>
      <name>incidentDate</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>277bf1b3-cbee-425f-af94-b7119b15d764</id>
      <masked>false</masked>
      <name>checkInLocality</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4fcd0f93-35bd-43aa-9754-c86075c806a9</id>
      <masked>false</masked>
      <name>incidentLocality</name>
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

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
