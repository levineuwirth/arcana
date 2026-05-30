//! Faithbound Judge // Sinner's Judgment
//!
//! Front face: `{1}{W}{W}` Spirit Soldier creature 4/4 with Defender, Flying, Vigilance.
//! At the beginning of your upkeep, if this creature has two or fewer judgment counters, put a
//! judgment counter on it. As long as it has three or more judgment counters, it can attack as
//! though it didn't have defender.
//! Disturb {5}{W}{W} (GAP: Disturb keyword not modeled in engine).
//!
//! Back face (Sinner's Judgment): Enchantment — Aura Curse. Enchant player.
//! At the beginning of your upkeep, put a judgment counter on this Aura. Then if there are three
//! or more judgment counters on it, enchanted player loses the game.
//! If Sinner's Judgment would be put into a graveyard from anywhere, exile it instead.
//!
//! GAP: Disturb keyword not modeled — no `KeywordAbility::Disturb`.
//! GAP: "can attack as though it didn't have defender" static override not expressible.
//! GAP: "enchanted player loses the game" (win condition trigger) not in Effect catalog.
//! GAP: "if would be put into a graveyard, exile it instead" replacement effect not modeled.
//! GAP: back-face-only triggered ability (Sinner's Judgment upkeep trigger) not modeled.
//! GAP: conditional check "two or fewer judgment counters" not expressible without counter-query API.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Faithbound Judge");
    let spirit_sub = reg.interner_mut().intern("Spirit");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit_sub);
    subtypes.0.insert(soldier_sub);

    // Pre-intern the counter name so it can be looked up at resolve time.
    let _judgment_id = reg.interner_mut().intern("judgment");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Defender,
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
        ],
        ..Default::default()
    };

    // Back face: Sinner's Judgment — Enchantment Aura Curse
    let back_name = reg.interner_mut().intern("Sinner's Judgment");
    let aura_sub = reg.interner_mut().intern("Aura");
    let curse_sub = reg.interner_mut().intern("Curse");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(aura_sub);
    back_subtypes.0.insert(curse_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::ENCHANTMENT.into(),
            subtypes: back_subtypes,
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face upkeep trigger: put a judgment counter on it
            // (conditional "two or fewer" check is a GAP — emitting unconditionally as best-effort)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: front_upkeep_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn front_upkeep_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if this creature has two or fewer judgment counters" — counter-query not available.
    // Emitting unconditional add as best-effort.
    let Some(judgment_id) = reg.interner().lookup("judgment") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(judgment_id),
        count: 1,
    }]
}
