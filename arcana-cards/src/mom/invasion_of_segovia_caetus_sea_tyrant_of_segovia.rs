//! Invasion of Segovia // Caetus, Sea Tyrant of Segovia
//!
//! Front face: Battle — Siege, {2}{U}, enters with 3 defense counters.
//! When this Siege enters, create two 1/1 blue Kraken creature tokens with trample.
//! Back face: Legendary Creature — Serpent, 4/4.
//! Back-face triggered ability: at the beginning of each end step, untap up to four target
//! creatures (face-gated to the back face, face 1).
//! GAP: "Noncreature spells you cast have convoke" — a static modifying how YOUR spells are
//!      cast, not a keyword on this creature; no spell-granting-convoke static primitive.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Segovia");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    // Pre-intern Kraken for token creation at resolve time.
    let _ = reg.interner_mut().intern("Kraken");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Caetus, Sea Tyrant of Segovia");
    let serpent_sub = reg.interner_mut().intern("Serpent");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(serpent_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 3,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: "At the beginning of each end step, untap up to four target
            // creatures." Face-gated to the back face (face 1).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: end_step_untap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(4),
                    controller: None,
                }],
            })
            .with_trigger_face_gate(2, 1),
    )
}

fn end_step_untap(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| {
            if let TargetChoice::Object(id) = t {
                Some(Effect::Untap { target: *id })
            } else {
                None
            }
        })
        .collect()
}

fn etb_create_tokens(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let kraken_name = reg.interner().lookup("Kraken").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(kraken_name);
    let token = TokenDefinition {
        name: kraken_name,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        keywords: vec![KeywordAbility::Trample],
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        abilities: vec![],
    };
    vec![
        Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        },
        Effect::CreateToken {
            controller: trig.controller,
            token,
        },
    ]
}
