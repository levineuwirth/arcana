//! Invasion of Kylem // Valor's Reach Tag Team
//!
//! Front face — Battle — Siege ({2}{R}{W}), enters with 5 defense counters:
//! When this Siege enters, up to two target creatures each get +2/+0 and gain
//! vigilance and haste until end of turn.
//! Back face — Sorcery: Create two 3/2 red and white Warrior creature tokens with
//! "Whenever this token and at least one other creature token attack, put a
//! +1/+1 counter on this token."
//!
//! Defeat→back-face is auto-wired by the engine SBA: when this Siege is defeated
//! it transforms in place, firing `SelfTransforms { to_face: Some(1) }`. The
//! back face's sorcery effect (create two 3/2 Warrior tokens) is wired on that
//! trigger. Front ETB (the +X/+0 pump) is face-gated to the battle face (0).
//!
//! GAP: the created tokens' own ability — "whenever this token and at least one
//!      other creature token attack, put a +1/+1 counter on this token" — has no
//!      "this and ≥1 other creature token attack" trigger condition; the tokens
//!      are created without it.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Kylem");
    let siege = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege);

    // Pre-intern Warrior for the back-face token creation.
    let _ = reg.interner_mut().intern("Warrior");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face — Sorcery; its token-creation effect fires on the defeat
    // transform (SelfTransforms{to_face:1}).
    let back_name = reg.interner_mut().intern("Valor's Reach Tag Team");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            .with_transform_back(back)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_pump,
                trigger_zones: Vec::new(),
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::UpTo(2),
                        controller: None,
                    },
                ],
            })
            // Back (Valor's Reach Tag Team): on defeat the Siege transforms to
            // its back (sorcery) face, which creates two 3/2 R/W Warrior tokens.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: back_create_warriors,
                trigger_zones: Vec::new(),
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn back_create_warriors(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let warrior = reg.interner().lookup("Warrior").expect("Warrior interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(warrior);
    let token = TokenDefinition {
        name: warrior,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        // GAP: "whenever this token and at least one other creature token
        // attack, put a +1/+1 counter on this token" — no such trigger
        // condition; token created without its self-counter ability.
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

fn etb_pump(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = Vec::new();
    for target in &trig.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::Pump {
                target: *id,
                power: 2,
                toughness: 0,
                duration: Duration::EndOfTurn,
                keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Haste],
            });
        }
    }
    effects
}
