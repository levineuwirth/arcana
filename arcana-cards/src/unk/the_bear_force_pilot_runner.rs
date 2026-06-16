//! The Bear Force Pilot Runner — `{1}{G}` Legendary 2/2 Bear Pilot Gamer.
//! "When The Bear Force Pilot Runner enters, create a 0/0 colorless Vehicle artifact
//!  token with flying, 'This creature gets +1/+1 for each Bear you control,' and crew 2."
//! "Ready to run."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Bear Force Pilot Runner");
    let bear = reg.interner_mut().intern("Bear");
    let pilot = reg.interner_mut().intern("Pilot");
    let gamer = reg.interner_mut().intern("Gamer");
    let _vehicle = reg.interner_mut().intern("Vehicle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(pilot);
    subtypes.0.insert(gamer);

    // GAP: "Ready to run" is not an available KeywordAbility variant (a partner-style
    //      commander keyword) — omitted.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_make_vehicle,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_make_vehicle(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let vehicle = match reg.interner().lookup("Vehicle") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vehicle);
    // GAP: the token's static "gets +1/+1 for each Bear you control" (a dynamic
    //      continuous static) and "crew 2" cannot be modeled on a TokenDefinition
    //      (only keywords + TriggeredAbilityDef abilities are expressible); the base
    //      0/0 colorless flying Vehicle artifact token is minted faithfully.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: vehicle,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
