<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add Traveler_ADT</name>
   <tag></tag>
   <elementGuidId>bc22296f-ad78-41b7-aabf-541c42694baf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;Traveler\&quot;: {\r\n        \&quot;birthDate\&quot;: \&quot;1999-02-22\&quot;,\r\n        \&quot;gender\&quot;: \&quot;Male\&quot;,\r\n        \&quot;PersonName\&quot;: {\r\n            \&quot;@type\&quot;: \&quot;PersonNameDetail\&quot;,\r\n            \&quot;Prefix\&quot;: \&quot;Mr\&quot;,\r\n            \&quot;Given\&quot;: \&quot;Px ADTOne\&quot;,\r\n            \&quot;Middle\&quot;: \&quot;MdNm \&quot;,\r\n            \&quot;Surname\&quot;: \&quot;SrNmOne\&quot;,\r\n            \&quot;Suffix\&quot;: \&quot;ADT\&quot;\r\n        },\r\n        \&quot;Telephone\&quot;: [\r\n            {\r\n                \&quot;@type\&quot;: \&quot;Telephone\&quot;,\r\n                \&quot;countryAccessCode\&quot;: \&quot;1\&quot;,\r\n                \&quot;areaCityCode\&quot;: \&quot;909\&quot;,\r\n                \&quot;phoneNumber\&quot;: \&quot;212456121\&quot;,\r\n                \&quot;extension\&quot;: \&quot;1243\&quot;,\r\n                \&quot;id\&quot;: \&quot;4\&quot;,\r\n                \&quot;cityCode\&quot;: \&quot;ORD\&quot;,\r\n                \&quot;role\&quot;: \&quot;Office\&quot;\r\n            }\r\n        ],\r\n        \&quot;Email\&quot;: [\r\n            {\r\n                \&quot;comment\&quot;: \&quot;One email Id\&quot;,\r\n                \&quot;value\&quot;: \&quot;TravelerOneOne@gmail.com\&quot;\r\n            },\r\n            {\r\n                \&quot;comment\&quot;: \&quot;Two email Id\&quot;,\r\n                \&quot;value\&quot;: \&quot;TravelerOneTwo@gmail.com\&quot;\r\n            }\r\n        ],\r\n        \&quot;TravelDocument\&quot;: [\r\n            {\r\n                \&quot;@type\&quot;: \&quot;TravelDocumentDetail\&quot;,\r\n                \&quot;docNumber\&quot;: \&quot;245968\&quot;,\r\n                \&quot;docType\&quot;: \&quot;Passport\&quot;,\r\n                \&quot;issueDate\&quot;: \&quot;2019-12-22\&quot;,\r\n                \&quot;expireDate\&quot;: \&quot;2027-02-22\&quot;,\r\n                \&quot;issueCountry\&quot;: \&quot;IND\&quot;,\r\n                \&quot;birthDate\&quot;: \&quot;1999-02-22\&quot;,\r\n                \&quot;birthCountry\&quot;: \&quot;IND\&quot;,\r\n                \&quot;Gender\&quot;: \&quot;Male\&quot;,\r\n                \&quot;PersonName\&quot;: {\r\n                    \&quot;@type\&quot;: \&quot;PersonName\&quot;,\r\n                    \&quot;Given\&quot;: \&quot;Px One\&quot;,\r\n                    \&quot;Middle\&quot;: \&quot;MdNm One\&quot;,\r\n                    \&quot;Surname\&quot;: \&quot;SrNmOne\&quot;\r\n                },\r\n                \&quot;IssuedForGeoPoliticalArea\&quot;: {\r\n                    \&quot;value\&quot;: \&quot;IND\&quot;\r\n                }\r\n            }\r\n        ]\r\n    }\r\n}&quot;,
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
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Version</name>
      <type>Main</type>
      <value>${Version}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.apim-a.zu2.qa.travelport.io/uat/${Version}/air/book/traveller/RES_DOMAIN/reservationworkbench/${reservationId}/travelers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Version</defaultValue>
      <description></description>
      <id>c5a59766-b379-4ce5-b5ba-f6fc7f1d2be9</id>
      <masked>false</masked>
      <name>Version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationId</defaultValue>
      <description></description>
      <id>1c60e568-5324-45ac-9dfe-ada22202a897</id>
      <masked>false</masked>
      <name>reservationId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>8c34ff4d-c6eb-4b66-9e96-1ce0bb5e72f8</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G</defaultValue>
      <description></description>
      <id>bcb437d0-b5e4-4dd6-8e4d-05b955b96b36</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
