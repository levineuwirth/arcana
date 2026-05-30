//! A-Dorothea, Vengeful Victim // A-Dorothea's Retribution
//!
//! Front: Legendary Creature — Spirit, {W}{U}, 4/4.
//! Flying.
//! When Dorothea, Vengeful Victim attacks or blocks, sacrifice it at end of combat.
//! Disturb {W}{U} (cast from graveyard transformed for disturb cost — not modeled, engine debt).
//!
//! Back: Enchantment — Aura (A-Dorothea's Retribution).
//! Enchant creature.
//! Enchanted creature has "Whenever this creature attacks, create a 4/4 white Spirit creature
//! token with flying that's tapped and attacking. Sacrifice that token at end of combat."
//! If Dorothea's Retribution would be put into a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb mechanic (cast from graveyard transformed) not modeled — engine debt.
//! GAP: Back face is an Aura Enchantment, not a creature; TypeLine::ENCHANTMENT used.
//! GAP: "Enchant creature" Aura attachment rules not modeled for transform-to-Aura shape.
//! GAP: "Enchanted creature has" ability — granting abilities to the enchanted creature via
//! transform-to-Aura is not modeled; back-face triggered ability not auto-installed.
//! GAP: "If Dorothea's Retribution would be put into a graveyard from anywhere, exile it instead"
//! — replacement effect not expressible; GAP'd.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Dorothea, Vengeful Victim");

    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("A-Dorothea's Retribution");
    let aura_sub = reg.interner_mut().intern("Aura");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white() | ColorSet::blue(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: when this attacks, sacrifice at end of combat (via DelayedAction)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: sac_at_end_of_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Front face: when this blocks, sacrifice at end of combat
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: sac_at_end_of_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: back-face-only triggered ability not modeled (Aura-granting attack trigger).
    )
}

fn sac_at_end_of_combat(
    _state: &arcana_core::state::GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Sacrifice,
    }]
}
