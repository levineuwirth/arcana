//! Nelly Borca, Impulsive Accuser — `{2}{R}{W}` 2/4 legendary Human
//! Detective with Vigilance.
//! "Whenever Nelly Borca attacks, suspect target creature. Then goad
//! all suspected creatures.
//! Whenever one or more creatures an opponent controls deal combat
//! damage to one or more of your opponents, you and the controller of
//! those creatures each draw a card."
//!
//! Abilities:
//! 1. Vigilance (keyword). (Scryfall also tags "Goad", but that is the
//!    ability's effect, not an evergreen keyword on the card.)
//! 2. SelfAttacks (target creature) → suspect it, then goad it.
//!    PARTIAL: "goad ALL suspected creatures" can only goad the
//!    just-suspected target (no script helper enumerates the suspected
//!    set); pre-existing suspected creatures aren't re-goaded.
//! 3. Opponent's creatures deal combat damage to your opponents → you
//!    draw. PARTIAL: "the controller of those creatures also draws"
//!    can't be expressed (no damaging-source-controller accessor).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nelly Borca, Impulsive Accuser");
    let human = reg.interner_mut().intern("Human");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: suspect_and_goad,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: you_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn suspect_and_goad(
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
    // Suspect the target, then goad it (now a suspected creature).
    // PARTIAL: "goad ALL suspected creatures" — no helper enumerates
    // the suspected set, so only the just-suspected target is goaded.
    vec![
        Effect::Suspect { target: *id },
        Effect::Goad {
            target: *id,
            goader: trig.controller,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn you_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // You draw a card. PARTIAL: "and the controller of those creatures
    // each draw a card" can't be expressed (no accessor for the
    // damaging source's controller).
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
