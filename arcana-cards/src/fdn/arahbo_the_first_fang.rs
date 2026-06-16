//! Arahbo, the First Fang — `{2}{W}` 2/2 Legendary Cat Avatar.
//! Other Cats you control get +1/+1 (static — GAP).
//! Whenever Arahbo or another nontoken Cat you control enters, create a 1/1
//! white Cat creature token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arahbo, the First Fang");
    let cat = reg.interner_mut().intern("Cat");
    let avatar = reg.interner_mut().intern("Avatar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(avatar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "Other Cats you control get +1/+1" — continuous anthem,
    // not a triggered/activated ability.

    let cat_filter = script::subtype_filter(reg, "Cat")
        .controlled_by(ControllerConstraint::You)
        .nontoken();

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: cat_filter,
                from: None,
                to: Zone::Battlefield,
            },
            intervening_if: None,
            effect: make_cat_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_cat_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let cat = reg.interner().lookup("Cat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: cat,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
