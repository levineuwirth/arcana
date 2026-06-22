//! Toxrill, the Corrosive — `{5}{B}{B}` 7/7 Legendary Slug Horror.
//! "At the beginning of each end step, put a slime counter on each creature you
//!  don't control."
//! "Creatures you don't control get -1/-1 for each slime counter on them." (static — GAP)
//! "Whenever a creature you don't control with a slime counter on it dies,
//!  create a 1/1 black Slug creature token."
//! "{U}{B}, Sacrifice a Slug: Draw a card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Toxrill, the Corrosive");
    let slug = reg.interner_mut().intern("Slug");
    let horror = reg.interner_mut().intern("Horror");
    let _slime = reg.interner_mut().intern("slime");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(slug);
    subtypes.0.insert(horror);

    let slug_filter = ObjectFilter::creature().with_subtype_sym(slug);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // 1. "At the beginning of each end step, put a slime counter on each
            //     creature you don't control."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: slime_each_opponent_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // 2. GAP static: "Creatures you don't control get -1/-1 for each
            //    slime counter on them." — dynamic continuous debuff, no demonstrated
            //    API surface.
            // 3. "Whenever a creature you don't control with a slime counter on it
            //     dies, create a 1/1 black Slug creature token."
            //    GAP fidelity: the "with a slime counter on it" restriction is not
            //    expressible on the ZoneChange filter.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: make_slug_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // 4. "{U}{B}, Sacrifice a Slug: Draw a card."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}{B}, Sacrifice a Slug: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{B}").expect("valid cost"),
                    sacrifice_other: Some(slug_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_one,
            }),
    )
}

fn slime_each_opponent_creature(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(slime) = reg.interner().lookup("slime").map(CounterKind::Named) else {
        return Vec::new();
    };
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: slime,
            count: 1,
        }),
    }]
}

fn make_slug_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let slug = reg.interner().lookup("Slug").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(slug);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: slug,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn draw_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 1,
    }]
}
