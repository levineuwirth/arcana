//! Karrthus, Tyrant of Jund — `{4}{B}{R}{G}` 7/7 Legendary Dragon.
//!
//! * Flying, haste.
//! * When Karrthus enters, gain control of all Dragons, then untap all
//!   Dragons.
//! * Other Dragon creatures you control have haste. (GAP'd — pure static
//!   continuous anthem ability, not triggered/activated.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karrthus, Tyrant of Jund");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_seize_dragons,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
        // GAP: "Other Dragon creatures you control have haste." — pure
        // static continuous anthem ability.
    )
}

/// ETB: gain control of all Dragons, then untap all Dragons.
fn etb_seize_dragons(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragons = script::ids_matching(
        state,
        &script::subtype_filter(reg, "Dragon"),
        trig.controller,
    );
    let mut effects: Vec<Effect> = Vec::new();
    for id in &dragons {
        effects.push(Effect::ChangeControl {
            target: *id,
            new_controller: trig.controller,
        });
    }
    for id in &dragons {
        effects.push(Effect::Untap { target: *id });
    }
    vec![Effect::Sequence(effects)]
}
