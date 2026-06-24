//! Namor the Sub-Mariner — `{1}{U}{U}` */4 Legendary Mutant Merfolk Villain.
//!
//! Oracle:
//! * Flying.
//! * Namor's power is equal to the number of Merfolk you control. (power `*`)
//! * Whenever you cast a noncreature spell with one or more blue mana symbols
//!   in its mana cost, create that many 1/1 blue Merfolk creature tokens.
//!
//! CDA: the `*` power counts a SUBTYPE (Merfolk you control) on an ASYMMETRIC
//! card (*/4) — wired at Layer 7a via `self_pt_from_match_asym` (power = Merfolk
//! count, toughness fixed 4) over a Merfolk-subtype filter built in the ETB fn,
//! which has the interner to name "Merfolk".
//!
//! The spell-cast trigger IS wired: the noncreature filter, and the "that
//! many" token count (number of blue mana symbols in the cast spell's cost),
//! read off the triggering spell's mana cost.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::events::GameEvent;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Namor the Sub-Mariner");
    let mutant = reg.interner_mut().intern("Mutant");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let villain = reg.interner_mut().intern("Villain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(merfolk);
    subtypes.0.insert(villain);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_merfolk_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Power = Merfolk you control; toughness fixed 4.
fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let merfolk =
        script::subtype_filter(reg, "Merfolk").controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match_asym(
            trig.source,
            merfolk,
            /*count_is_power=*/ true,
            /*other_fixed=*/ 4,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn make_merfolk_tokens(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // N = number of blue mana symbols in the cast spell's mana cost.
    let spell_id = match trig.trigger_event {
        GameEvent::SpellCast { object_id, .. } => object_id,
        _ => return Vec::new(),
    };
    let blue_pips: usize = state
        .objects
        .get(spell_id)
        .and_then(|o| o.characteristics.mana_cost.as_ref())
        .map(|cost| {
            cost.components
                .iter()
                .filter(|c| (c.colors().0 & ColorSet::BLUE) != 0)
                .count()
        })
        .unwrap_or(0);
    if blue_pips == 0 {
        return Vec::new();
    }

    let merfolk = match reg.interner().lookup("Merfolk") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    let token = TokenDefinition {
        name: merfolk,
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    std::iter::repeat_with(|| Effect::CreateToken {
        controller: trig.controller,
        token: token.clone(),
    })
    .take(blue_pips)
    .collect()
}
