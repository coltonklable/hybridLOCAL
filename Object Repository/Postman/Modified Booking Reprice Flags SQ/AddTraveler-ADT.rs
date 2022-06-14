<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddTraveler-ADT</name>
   <tag></tag>
   <elementGuidId>f78a1b5a-3405-4fc8-a459-c3a12a4bced9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Traveler\&quot;: {\n        \&quot;passengerTypeCode\&quot;: \&quot;ADT\&quot;,\n        \&quot;dob\&quot;: \&quot;1990-12-22\&quot;,\n        \&quot;gender\&quot;: \&quot;Male\&quot;,\n        \&quot;PersonName\&quot;: {\n            \&quot;@type\&quot;: \&quot;PersonNameDetail\&quot;,\n            \&quot;Prefix\&quot;: \&quot;Mr\&quot;,\n            \&quot;Given\&quot;: \&quot;James\&quot;,\n            \&quot;Middle\&quot;: \&quot;Clif\&quot;,\n            \&quot;Surname\&quot;: \&quot;Harkenss\&quot;,\n            \&quot;Suffix\&quot;: \&quot;ADT\&quot;,\n            \&quot;Gender\&quot;: \&quot;Male\&quot;\n        },\n        \&quot;Telephone\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;Telephone\&quot;,\n                \&quot;countryAccessCode\&quot;: \&quot;1\&quot;,\n                \&quot;areaCityCode\&quot;: \&quot;909\&quot;,\n                \&quot;phoneNumber\&quot;: \&quot;212456121\&quot;,\n                \&quot;extension\&quot;: \&quot;1243\&quot;,\n                \&quot;id\&quot;: \&quot;Tel4\&quot;,\n                \&quot;cityCode\&quot;: \&quot;ORD\&quot;,\n                \&quot;role\&quot;: \&quot;Home\&quot;\n            }\n        ],\n        \&quot;Address\&quot;: [\n                    {\n                        \&quot;@type\&quot;: \&quot;Address\&quot;,\n                        \&quot;id\&quot;: \&quot;address_1\&quot;,\n                        \&quot;role\&quot;: \&quot;Home\&quot;,\n                        \&quot;Street\&quot;: \&quot;Enterprise Lane\&quot;,\n                        \&quot;AddressLine\&quot;: [\n                            \&quot;1234 Enterprise Lane\&quot;\n                        ],\n                        \&quot;City\&quot;: \&quot;La Mirada\&quot;,\n                        \&quot;County\&quot;: \&quot;Los Angeles\&quot;,\n                        \&quot;StateProv\&quot;: {\n                            \&quot;name\&quot;: \&quot;California\&quot;,\n                            \&quot;value\&quot;: \&quot;CA\&quot;\n                        },\n                        \&quot;Country\&quot;: {\n                            \&quot;id\&quot;: \&quot;country_2\&quot;,\n                            \&quot;name\&quot;: \&quot;United States\&quot;,\n                            \&quot;codeContext\&quot;: \&quot;CC\&quot;,\n                            \&quot;value\&quot;: \&quot;US\&quot;\n                        },\n                        \&quot;PostalCode\&quot;: \&quot;90638\&quot;\n                    }\n                ],\n        \&quot;TravelDocument\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;TravelDocument\&quot;,\n                \&quot;docNumber\&quot;: \&quot;H294F4\&quot;,\n                \&quot;docType\&quot;: \&quot;Passport\&quot;,\n                \&quot;expireDate\&quot;: \&quot;2027-02-22\&quot;,\n                \&quot;issueCountry\&quot;: \&quot;IND\&quot;,\n                \&quot;birthDate\&quot;: \&quot;1990-12-22\&quot;,\n                \&quot;Gender\&quot;: \&quot;Male\&quot;,\n                \&quot;PersonName\&quot;: {\n                    \&quot;@type\&quot;: \&quot;PersonName\&quot;,\n                    \&quot;Given\&quot;: \&quot;Meera\&quot;,\n                    \&quot;Middle\&quot;: \&quot;Mohideen\&quot;,\n                    \&quot;Surname\&quot;: \&quot;Traveler\&quot;\n                }\n            }\n        ],\n        \&quot;Email\&quot;: [\n            {\n                \&quot;comment\&quot;: \&quot;Primary Email Id\&quot;,\n                \&quot;value\&quot;: \&quot;TravelerOne@gmail.com\&quot;\n            }\n        ]\n    }\n}&quot;,
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
      <name>E2ETrackingID</name>
      <type>Main</type>
      <value>${guid}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP</name>
      <type>Main</type>
      <value>${XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9}</value>
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
   <restUrl>${ts_environment}/${version}/air/book/traveler/reservationworkbench/${reservationIdSq}/travelers</restUrl>
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
      <id>154a7323-721a-4cec-b563-40337be229c9</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>e2b3ba54-a44f-4caf-880e-80e7ad3e8322</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdSq</defaultValue>
      <description></description>
      <id>533f2029-8b24-4b22-8588-93ea16525cb7</id>
      <masked>false</masked>
      <name>reservationIdSq</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>d0b6cb9e-d522-4c09-9016-2d34edbe9395</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>886829bd-27b9-4378-a672-a5fa3a9b9d7f</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</defaultValue>
      <description></description>
      <id>f0ecdbd2-58cf-49bf-b0bf-0a4857ea95ab</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
