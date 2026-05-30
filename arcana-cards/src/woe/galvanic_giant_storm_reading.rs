//! Galvanic Giant // Storm Reading — `{3}{U}` Giant Wizard creature 3/3
//! with the Adventure face "Storm Reading" (`{5}{U}{U}` instant,
//! "Draw four cards, then discard two cards."). Wilds of Eldraine rare.
//!
//! The creature face has a triggered ability: "Whenever you cast a spell
//! with mana value 5 or greater, tap target creature an opponent controls
//! and put a stun counter on it."
//!
//! Adventure face: Draw 4, discard 2.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Galvanic Giant");
    let giant_sub = reg.interner_mut().intern("Giant");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant_sub);
    subtypes.0.insert(wizard_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid creature cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Storm Reading");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid adventure cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Draw four cards, then discard two cards.".into(),
        target_requirements: vec![],
        modal: None,
        effect: storm_reading_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    // Triggered ability: "Whenever you cast a spell with mana value 5 or greater,
    // tap target creature an opponent controls and put a stun counter on it."
    let opp_creature_filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::Opponent);

    reg.register(
        CardDefinition::new(name, chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_min_cmc(5)),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_high_cmc_spell_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(opp_creature_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
    )
}

fn storm_reading_resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: entry.controller, count: 4 },
        Effect::Discard {
            player: entry.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}

fn on_high_cmc_spell_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        },
    ]
}
