//! Bilbo, Luckwearer // Burglar's Plot — `{1}{U}` // `{4}{U}` blue Adventure creature.
//! Legendary Creature — Halfling Rogue. 1/1. Bilbo can't be blocked.
//! Whenever Bilbo deals combat damage to a player, draw a card, then discard a card.
//! Adventure (Burglar's Plot — Sorcery): Exchange control of two target nonland permanents that share a card type.
//! GAP: "can't be blocked" — no KeywordAbility for unblockable (engine supports skulk, fear, intimidate, shadow, horsemanship).
//! GAP: Burglar's Plot "exchange control" — ChangeControl swaps to you; exchange-mutual not in catalog.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bilbo, Luckwearer");
    let adv_name = reg.interner_mut().intern("Burglar's Plot");
    let halfling_sub = reg.interner_mut().intern("Halfling");
    let rogue_sub = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling_sub);
    subtypes.0.insert(rogue_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "can't be blocked" keyword not available
        keywords: vec![],
        ..Default::default()
    };
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Exchange control of two target nonland permanents that share a card type.".into(),
        target_requirements: vec![
            TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            },
            TargetRequirement {
                filter: TargetFilter::Permanent(
                    ObjectFilter::permanent().without_types(TypeLine::LAND.into()),
                ),
                count: TargetCount::Exactly(1),
                controller: None,
            },
        ],
        modal: None,
        effect: burglar_s_plot_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };
    reg.register(
        CardDefinition::new(name, main_chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    target_filter: arcana_core::targets::TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: bilbo_deals_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn bilbo_deals_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::Discard { player: trig.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}

fn burglar_s_plot_resolve(
    _state: &GameState,
    _entry: &StackEntry,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exchange control" — mutual swap not in catalog; ChangeControl only goes one direction
    Vec::new()
}
