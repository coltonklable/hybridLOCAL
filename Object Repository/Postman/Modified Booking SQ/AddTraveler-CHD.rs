<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddTraveler-CHD</name>
   <tag></tag>
   <elementGuidId>82744497-943a-4748-94c9-8f9b8d86b248</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Traveler\&quot;: {\n        \&quot;passengerTypeCode\&quot;: \&quot;CHD\&quot;,\n        \&quot;dob\&quot;: \&quot;2015-12-22\&quot;,\n        \&quot;gender\&quot;: \&quot;Male\&quot;,\n        \&quot;PersonName\&quot;: {\n            \&quot;@type\&quot;: \&quot;PersonNameDetail\&quot;,\n            \&quot;Prefix\&quot;: \&quot;Master\&quot;,\n            \&quot;Given\&quot;: \&quot;Mike\&quot;,\n            \&quot;Middle\&quot;: \&quot;Lafe\&quot;,\n            \&quot;Surname\&quot;: \&quot;Harkenss\&quot;,\n            \&quot;Suffix\&quot;: \&quot;CHD\&quot;,\n            \&quot;Gender\&quot;: \&quot;Male\&quot;\n        },\n        \&quot;Telephone\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;Telephone\&quot;,\n                \&quot;countryAccessCode\&quot;: \&quot;1\&quot;,\n                \&quot;areaCityCode\&quot;: \&quot;909\&quot;,\n                \&quot;phoneNumber\&quot;: \&quot;212456121\&quot;,\n                \&quot;extension\&quot;: \&quot;1243\&quot;,\n                \&quot;id\&quot;: \&quot;Tel4\&quot;,\n                \&quot;cityCode\&quot;: \&quot;ORD\&quot;,\n                \&quot;role\&quot;: \&quot;Home\&quot;\n            }\n        ],\n        \&quot;Address\&quot;: [\n                    {\n                        \&quot;@type\&quot;: \&quot;Address\&quot;,\n                        \&quot;id\&quot;: \&quot;address_1\&quot;,\n                        \&quot;role\&quot;: \&quot;Home\&quot;,\n                        \&quot;Street\&quot;: \&quot;Enterprise Lane\&quot;,\n                        \&quot;AddressLine\&quot;: [\n                            \&quot;1234 Enterprise Lane\&quot;\n                        ],\n                        \&quot;City\&quot;: \&quot;La Mirada\&quot;,\n                        \&quot;County\&quot;: \&quot;Los Angeles\&quot;,\n                        \&quot;StateProv\&quot;: {\n                            \&quot;name\&quot;: \&quot;California\&quot;,\n                            \&quot;value\&quot;: \&quot;CA\&quot;\n                        },\n                        \&quot;Country\&quot;: {\n                            \&quot;id\&quot;: \&quot;country_2\&quot;,\n                            \&quot;name\&quot;: \&quot;United States\&quot;,\n                            \&quot;codeContext\&quot;: \&quot;CC\&quot;,\n                            \&quot;value\&quot;: \&quot;US\&quot;\n                        },\n                        \&quot;PostalCode\&quot;: \&quot;90638\&quot;\n                    }\n                ],\n        \&quot;TravelDocument\&quot;: [\n            {\n                \&quot;@type\&quot;: \&quot;TravelDocument\&quot;,\n                \&quot;docNumber\&quot;: \&quot;H294F4\&quot;,\n                \&quot;docType\&quot;: \&quot;Passport\&quot;,\n                \&quot;expireDate\&quot;: \&quot;2027-02-22\&quot;,\n                \&quot;issueCountry\&quot;: \&quot;IND\&quot;,\n                \&quot;birthDate\&quot;: \&quot;2015-12-22\&quot;,\n                \&quot;Gender\&quot;: \&quot;Male\&quot;,\n                \&quot;PersonName\&quot;: {\n                    \&quot;@type\&quot;: \&quot;PersonName\&quot;,\n                    \&quot;Given\&quot;: \&quot;Meera\&quot;,\n                    \&quot;Middle\&quot;: \&quot;Mohideen\&quot;,\n                    \&quot;Surname\&quot;: \&quot;Traveler\&quot;\n                }\n            }\n        ],\n        \&quot;Email\&quot;: [\n            {\n                \&quot;comment\&quot;: \&quot;Primary Email Id\&quot;,\n                \&quot;value\&quot;: \&quot;TravelerOne@gmail.com\&quot;\n            }\n        ]\n    }\n}&quot;,
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
      <id>5b025bc9-5e22-49f3-bde2-ef6f8ddf3035</id>
      <masked>false</masked>
      <name>ts_environment</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>7758127f-d3ca-477c-89d9-c9bac5490c36</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.reservationIdSq</defaultValue>
      <description></description>
      <id>8a4ed1c4-e843-49b1-a969-f50a88455be7</id>
      <masked>false</masked>
      <name>reservationIdSq</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>e2f11a77-7ea6-48db-8df0-d6b5d53e2c6c</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.guid</defaultValue>
      <description></description>
      <id>bdcebc8a-be02-493c-a46a-ffbba4b85ea7</id>
      <masked>false</masked>
      <name>guid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</defaultValue>
      <description></description>
      <id>580cb02d-fd77-450b-8c41-7955c6a82c37</id>
      <masked>false</masked>
      <name>XAUTH_TRAVELPORT_ACCESSGROUP_1G_69Z9</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
