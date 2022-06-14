<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Traveler 2</name>
   <tag></tag>
   <elementGuidId>79252539-58ed-447d-9f2a-684f6e0da4db</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Traveler\&quot;: {\n    \&quot;dob\&quot;: \&quot;1995-03-16\&quot;,\n    \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;,\n    \&quot;PersonName\&quot;: {\n      \&quot;@type\&quot;: \&quot;PersonNameDetail\&quot;,\n      \&quot;Prefix\&quot;: \&quot;Mr\&quot;,\n      \&quot;Given\&quot;: \&quot;Tat\&quot;,\n      \&quot;Middle\&quot;: \&quot;Rex\&quot;,\n      \&quot;Surname\&quot;: \&quot;Pro\&quot;,\n      \&quot;Suffix\&quot;: \&quot;ADT\&quot;\n    },\n    \&quot;Telephone\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;Telephone\&quot;,\n        \&quot;countryAccessCode\&quot;: \&quot;1\&quot;,\n        \&quot;areaCityCode\&quot;: \&quot;381\&quot;,\n        \&quot;phoneNumber\&quot;: \&quot;382382555\&quot;,\n        \&quot;extension\&quot;: \&quot;7698\&quot;,\n        \&quot;id\&quot;: \&quot;4\&quot;,\n        \&quot;cityCode\&quot;: \&quot;ORD\&quot;,\n        \&quot;role\&quot;: \&quot;Office\&quot;\n      }\n    ],\n    \&quot;Email\&quot;: [\n      {\n        \&quot;comment\&quot;: \&quot;Primary Email Id\&quot;,\n        \&quot;value\&quot;: \&quot;Johnny_Greenfelder@gmail.com\&quot;\n      }\n    ],\n    \&quot;CustomerLoyalty\&quot;: [\n      {\n        \&quot;supplier\&quot;: \&quot;DL\&quot;,\n        \&quot;value\&quot;: \&quot;DL2071983684\&quot;\n      }\n    ],\n    \&quot;TravelDocument\&quot;: [\n      {\n        \&quot;@type\&quot;: \&quot;TravelDocument\&quot;,\n        \&quot;docNumber\&quot;: \&quot;620023\&quot;,\n        \&quot;docType\&quot;: \&quot;Passport\&quot;,\n        \&quot;expireDate\&quot;: \&quot;2025-09-16\&quot;,\n        \&quot;issueCountry\&quot;: \&quot;BZ\&quot;,\n        \&quot;birthDate\&quot;: \&quot;1995-09-16\&quot;,\n        \&quot;birthCountry\&quot;: \&quot;GH\&quot;,\n        \&quot;Gender\&quot;: \&quot;Male\&quot;,\n        \&quot;PersonName\&quot;: {\n          \&quot;@type\&quot;: \&quot;PersonName\&quot;,\n          \&quot;Prefix\&quot;: \&quot;Mr\&quot;,\n          \&quot;Given\&quot;: \&quot;Tatyana\&quot;,\n          \&quot;Middle\&quot;: \&quot;Rex\&quot;,\n          \&quot;Surname\&quot;: \&quot;Prosacco\&quot;\n        }\n      }\n    ]\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>trackingId</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${ts_environment}/${version}/air/book/traveler/reservationworkbench/${reservationIdUa}/travelers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ts_environment</defaultValue>
      <description></description>
      <id>213d0a22-391c-4aa8-a264-db7929b89788</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>e4f1a11b-9ef0-40b3-87e1-250169c9d6b8</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdUa</defaultValue>
      <description></description>
      <id>9f0b7756-9433-4693-81cc-e4835ffcbafe</id>
      <masked>false</masked>
      <name>reservationIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>9af06875-3808-496c-90d4-44d9de7c0d0c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>5fa5582b-55e3-4838-9150-b833bf26d326</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>659a5e8e-f80a-44f5-ad57-b9a8423d7eab</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
