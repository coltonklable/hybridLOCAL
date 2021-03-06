<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Traveler</name>
   <tag></tag>
   <elementGuidId>0fd51413-38a4-4696-af32-731451e5b0da</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;Traveler\&quot; : {\n    \&quot;dob\&quot; : \&quot;1984-02-22\&quot;,\n    \&quot;gender\&quot; : \&quot;Male\&quot;,\n    \&quot;passengerTypeCode\&quot; : \&quot;ADT\&quot;,\n    \&quot;PersonName\&quot; : {\n      \&quot;@type\&quot; : \&quot;PersonNameDetail\&quot;,\n      \&quot;Prefix\&quot; : \&quot;Mr\&quot;,\n      \&quot;Given\&quot; : \&quot;JAMES\&quot;,\n      \&quot;Surname\&quot; : \&quot;HARKNESS\&quot;\n    },\n    \&quot;Telephone\&quot; : [ {\n      \&quot;@type\&quot; : \&quot;Telephone\&quot;,\n      \&quot;countryAccessCode\&quot; : \&quot;1\&quot;,\n      \&quot;areaCityCode\&quot; : \&quot;909\&quot;,\n      \&quot;phoneNumber\&quot; : \&quot;212456121\&quot;,\n      \&quot;extension\&quot; : \&quot;1243\&quot;,\n      \&quot;id\&quot; : \&quot;4\&quot;,\n      \&quot;cityCode\&quot; : \&quot;ORD\&quot;,\n      \&quot;role\&quot; : \&quot;Office\&quot;\n    },\n    {\n      \&quot;@type\&quot; : \&quot;Telephone\&quot;,\n      \&quot;countryAccessCode\&quot; : \&quot;1\&quot;,\n      \&quot;areaCityCode\&quot; : \&quot;303\&quot;,\n      \&quot;phoneNumber\&quot; : \&quot;111456121\&quot;,\n      \&quot;extension\&quot; : \&quot;1243\&quot;,\n      \&quot;id\&quot; : \&quot;4\&quot;,\n      \&quot;cityCode\&quot; : \&quot;ORD\&quot;,\n      \&quot;role\&quot; : \&quot;Office\&quot;\n    } ],\n    \&quot;Email\&quot; : [ {\n      \&quot;comment\&quot; : \&quot;Primary Email Id\&quot;,\n      \&quot;value\&quot; : \&quot;TravelerOne@gmail.com\&quot;\n    } ]\n  }\n}&quot;,
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
      <name>Accept-Version</name>
      <type>Main</type>
      <value>${version}</value>
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
      <id>a86692f9-8206-4c63-a6e6-7f10e7da76fd</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>7dbc2c0f-443b-4210-8d2f-127df3ed0e8d</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdUa</defaultValue>
      <description></description>
      <id>73c460e7-0e8d-4b35-956e-84a6dbc140da</id>
      <masked>false</masked>
      <name>reservationIdUa</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>d52f41e6-7341-4acb-9ee2-0bbd0c11faa5</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>cea8f1de-ba20-44a0-a915-0dc349254492</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</defaultValue>
      <description></description>
      <id>6f9ea422-0823-4fb8-a78f-4ca2830a0b0d</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_XB7</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
