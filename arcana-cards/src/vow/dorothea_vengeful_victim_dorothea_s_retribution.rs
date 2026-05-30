//! Dorothea, Vengeful Victim // Dorothea's Retribution
//! {W}{U} Legendary Creature — Spirit (4/4) / Enchantment — Aura
//! Front: Flying. When Dorothea attacks or blocks, sacrifice it at end of combat.
//!        Disturb {1}{W}{U}.
//! Back (Aura): Enchant creature. Enchanted creature has "Whenever this creature attacks,
//!              create a 4/4 white Spirit with flying that's tapped and attacking. Sacrifice at end of combat."
//!              If Dorothea's Retribution would be put into a graveyard, exile it instead.
//! GAP: Disturb (cast from graveyard transformed) is not modeled.
//! GAP: Back face Aura ETB ("enchant creature") and grant-triggered-ability effect not modeled.
//! GAP: "at end of combat" — modeled as NextEndStep (no EndOfCombat in DelayedWhen).
//! GAP: "If Dorothea's Retribution would be put into a graveyard, exile it instead" (replacement effect) not modeled.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dorothea, Vengeful Victim");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Dorothea's Retribution");
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
            // When Dorothea attacks, sacrifice it at end of combat (modeled as NextEndStep)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: sac_at_end_of_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // When Dorothea blocks, sacrifice it at end of combat (modeled as NextEndStep)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: sac_at_end_of_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: back-face Aura grant-triggered-ability effect not modeled
    )
}

fn sac_at_end_of_combat(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "at end of combat" modeled as NextEndStep (no EndOfCombat in DelayedWhen).
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::Sacrifice,
    }]
}
