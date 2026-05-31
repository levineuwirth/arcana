//! Riling Dawnbreaker // Signaling Roar — `{4}{W}` white Dragon 3/4
//! with flying and vigilance, plus the Adventure face "Signaling Roar"
//! (`{1}{W}` Sorcery — Omen, "Create a 2/2 white Soldier creature
//! token.").
//!
//! Creature face: Flying, vigilance. At the beginning of combat on your
//! turn, another target creature you control gets +1/+0 until end of
//! turn.
//!
//! Adventure face (Signaling Roar): Create a 2/2 white Soldier creature
//! token. (The Omen "shuffle this card into its owner's library"
//! after-effect is the Omen-variant analogue of CR 715.4 exile; the
//! Adventure engine wiring exiles the card on resolution — GAP: the
//! shuffle-into-library variant of the post-resolution destination is
//! not separately modeled.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::state::GameState;
use arcana_core::stack::StackEntry;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::layers::Duration;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riling Dawnbreaker");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    // Pre-intern the token subtype so the resolver can look it up.
    let _soldier_sub = reg.interner_mut().intern("Soldier");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);

    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // Adventure face "Signaling Roar" — {1}{W} Sorcery (Omen).
    let adv_name = reg.interner_mut().intern("Signaling Roar");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a 2/2 white Soldier creature token.".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: signaling_roar_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, main_chars)
            .with_adventure(adventure)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: begin_combat_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn begin_combat_pump(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn signaling_roar_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(soldier) = reg.interner().lookup("Soldier") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(soldier);
    vec![Effect::CreateToken {
        controller: entry.controller,
        token: TokenDefinition {
            name: soldier,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
