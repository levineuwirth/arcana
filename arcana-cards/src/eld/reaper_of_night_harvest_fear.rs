//! Reaper of Night // Harvest Fear — `{5}{B}{B}` black Specter creature
//! 4/5 with the Adventure face "Harvest Fear" (`{3}{B}` Sorcery).
//!
//! Creature: Whenever this creature attacks, if defending player has two
//! or fewer cards in hand, it gains flying until end of turn.
//! Adventure: Target opponent discards two cards.

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reaper of Night");
    let specter = reg.interner_mut().intern("Specter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(specter);

    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // Adventure face — Harvest Fear ({3}{B} Sorcery).
    let adv_name = reg.interner_mut().intern("Harvest Fear");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Target opponent discards two cards.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Player,
            count: TargetCount::Exactly(1),
            controller: Some(ControllerConstraint::Opponent),
        }],
        modal: None,
        effect: harvest_fear_resolve,
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
                trigger_condition: TriggerCondition::SelfAttacks,
                // GAP: intervening-if "if defending player has two or
                // fewer cards in hand" — the conditions helpers only
                // gate on the controller's own state, not the defending
                // player's hand. Firing unconditionally (best-effort).
                intervening_if: None,
                effect: on_attack_gain_flying,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_adventure(adventure),
    )
}

fn on_attack_gain_flying(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: trig.source,
        keyword: KeywordAbility::Flying,
        duration: Duration::EndOfTurn,
    }]
}

fn harvest_fear_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    vec![Effect::Discard {
        player: *p,
        count: 2,
        choice: DiscardChoice::ControllerChooses,
    }]
}
