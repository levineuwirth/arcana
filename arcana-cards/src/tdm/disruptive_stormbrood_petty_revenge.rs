//! Disruptive Stormbrood // Petty Revenge — `{4}{G}` Creature — Dragon 3/3 // Sorcery — Omen
//!
//! Creature: Flying. When this creature enters, destroy up to one target artifact or enchantment.
//! Adventure "Petty Revenge" (`{1}{B}` sorcery): Destroy target creature with power 3 or less.
//! Note: adventure sorcery colors are B but this is an Omen adventure subtype.
//! (Then shuffle this card into its owner's library — handled by engine adventure exile pipeline.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disruptive Stormbrood");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid creature cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Petty Revenge");
    let omen_sub = reg.interner_mut().intern("Omen");
    let mut adv_subtypes = SubtypeSet::default();
    adv_subtypes.0.insert(omen_sub);
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid adventure cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        subtypes: adv_subtypes,
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Destroy target creature with power 3 or less.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Permanent(
                ObjectFilter::creature().with_max_power(3),
            ),
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: petty_revenge_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            // ETB trigger: destroy up to one target artifact or enchantment.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_destroy_artifact_or_enchantment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types_any(TypeLine(
                            TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
                        )),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn petty_revenge_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn etb_destroy_artifact_or_enchantment(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}
