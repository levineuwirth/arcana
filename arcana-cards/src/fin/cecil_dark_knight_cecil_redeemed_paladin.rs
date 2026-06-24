//! Cecil, Dark Knight // Cecil, Redeemed Paladin — `{B}` Legendary Human Knight 2/3.
//!
//! Front face (Cecil, Dark Knight):
//! - Deathtouch
//! - Darkness — Whenever Cecil deals damage, you lose that much life. Then if your life
//!   total is less than or equal to half your starting life total, untap Cecil and
//!   transform it.
//!
//! Back face (Cecil, Redeemed Paladin):
//! - Lifelink
//! - Protect — Whenever Cecil attacks, other attacking creatures gain indestructible until end of turn.
//!
//! # GAPs
//! - "Protect" / "Darkness" are flavor names, not standard keyword abilities; omitted as
//!   keywords (the abilities themselves are wired as triggered abilities below).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cecil, Dark Knight");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);

    // Cecil deals the "Darkness" damage as its single named legendary; the
    // combat/non-combat damage trigger is restricted to this source by name.
    let self_name = reg.interner().lookup("Cecil, Dark Knight");
    let self_filter = ObjectFilter { name: self_name, ..ObjectFilter::default() };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    // Back face: Cecil, Redeemed Paladin
    let back_name = reg.interner_mut().intern("Cecil, Redeemed Paladin");
    let human_sub2 = reg.interner_mut().intern("Human");
    let knight_sub2 = reg.interner_mut().intern("Knight");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub2);
    back_subtypes.0.insert(knight_sub2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Lifelink],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face — "Darkness": whenever Cecil deals damage (to anything),
            // you lose that much life; then if your life is <= half your starting
            // life total, untap Cecil and transform it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: self_filter,
                    target_filter: TargetFilter::AnyTarget,
                    combat_only: false,
                },
                intervening_if: None,
                effect: darkness_lose_life_and_maybe_flip,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face — "Protect": whenever Cecil attacks, other attacking
            // creatures gain indestructible until end of turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: protect_other_attackers,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Trigger 1 fires only on the front face; trigger 2 only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1),
    )
}

fn darkness_lose_life_and_maybe_flip(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = trig.damage_amount().unwrap_or(0);
    let you = trig.controller;
    let mut effects = Vec::new();
    if amount > 0 {
        effects.push(Effect::LoseLife { player: you, amount });
    }
    // "Then if your life total is less than or equal to half your starting
    // life total, untap Cecil and transform it." Life loss above is applied
    // before this gate evaluates because effects resolve in order.
    let starting = state.format.starting_life;
    let life_after = script::life(state, you) - amount as i32;
    if life_after * 2 <= starting {
        effects.push(Effect::Untap { target: trig.source });
        effects.push(Effect::Transform { target: trig.source });
    }
    effects
}

fn protect_other_attackers(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let others = script::ids_matching(
        state,
        &ObjectFilter::creature().attacking_only(),
        trig.controller,
    )
    .into_iter()
    .filter(|id| *id != trig.source)
    .collect::<Vec<_>>();
    vec![Effect::ForEach {
        targets: others,
        effect: Box::new(Effect::GrantKeyword {
            target: NULL_OBJECT_ID,
            keyword: KeywordAbility::Indestructible,
            duration: Duration::EndOfTurn,
        }),
    }]
}
