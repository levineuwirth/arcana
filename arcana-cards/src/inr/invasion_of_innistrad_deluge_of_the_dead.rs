//! Invasion of Innistrad // Deluge of the Dead — `{2}{B}{B}` Battle — Siege.
//!
//! Front (Battle): Flash; when this Siege enters, target creature an opponent
//! controls gets -13/-13 until end of turn.
//!
//! Back (Enchantment): When this enchantment enters, create two 2/2 black
//! Zombie creature tokens. {2}{B}: Exile target card from a graveyard. If it
//! was a creature card, create a 2/2 black Zombie creature token.
//!
//! Back-face "when this enchantment enters" is wired on the
//! `SelfTransforms{to_face:Some(1)}` trigger (fires when the Siege is defeated
//! and becomes the enchantment). The `{2}{B}` exile activated ability is
//! face-gated to face 1; the conditional "if it was a creature card, create a
//! Zombie" is resolved by inspecting the exiled target's type at resolve time.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry, EntersWithSpec,
};
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
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Innistrad");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);
    // Pre-intern Zombie for back-face token creation at resolve time.
    let _zombie_sub = reg.interner_mut().intern("Zombie");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    // Back face: Deluge of the Dead (Enchantment)
    let back_name = reg.interner_mut().intern("Deluge of the Dead");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::ENCHANTMENT.into(),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 7,
            })
            .with_transform_back(back)
            // Front ETB trigger: target creature an opponent controls gets -13/-13.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_minus13,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            })
            // Back-face "when this enchantment enters" → create two 2/2 black
            // Zombie tokens. Fires on the defeat-transform to the back face.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfTransforms { to_face: Some(1) },
                intervening_if: None,
                effect: back_make_two_zombies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face activated ability: "{2}{B}: Exile target card from a
            // graveyard. If it was a creature card, create a 2/2 black Zombie."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{B}: Exile target card from a graveyard. If it was a creature card, create a 2/2 black Zombie creature token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::new(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1), // back (enchantment) face only
                effect: back_exile_then_maybe_zombie,
            }),
    )
}

fn etb_minus13(
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
    vec![Effect::Pump {
        target: *id,
        power: -13,
        toughness: -13,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

/// A 2/2 black Zombie creature token.
fn zombie_token(reg: &CardRegistry) -> TokenDefinition {
    let zombie = reg.interner().lookup("Zombie")
        .expect("Zombie interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    TokenDefinition {
        name: zombie,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn back_make_two_zombies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::CreateToken { controller: trig.controller, token: zombie_token(reg) },
        Effect::CreateToken { controller: trig.controller, token: zombie_token(reg) },
    ]
}

fn back_exile_then_maybe_zombie(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // "If it was a creature card, create a 2/2 black Zombie." Inspect the
    // exiled card's type while it is still in the graveyard.
    let was_creature = state.objects.get(*id).is_some_and(|o| o.is_creature());
    let mut effects = vec![Effect::ExileFromGraveyard { target: *id }];
    if was_creature {
        effects.push(Effect::CreateToken {
            controller: ctx.controller,
            token: zombie_token(reg),
        });
    }
    effects
}
