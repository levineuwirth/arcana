//! The Gitrog Monster — `{3}{B}{G}` 6/6 Legendary Frog Horror with Deathtouch.
//! "At the beginning of your upkeep, sacrifice The Gitrog Monster unless you
//! sacrifice a land." — wired as an optional sacrifice payment: the controller
//! may sacrifice a land (the payment); declining sacrifices The Gitrog Monster
//! itself (the penalty).
//! "You may play an additional land on each of your turns." (GAP — static.)
//! "Whenever one or more land cards are put into your graveyard from
//! anywhere, draw a card."

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Gitrog Monster");
    let frog = reg.interner_mut().intern("Frog");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // Upkeep "sacrifice ~ unless you sacrifice a land" — optional
            // sacrifice payment (sacrifice a land, else sacrifice this).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_sac_unless,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: draw_on_land_to_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn upkeep_sac_unless(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "sacrifice The Gitrog Monster unless you sacrifice a land" — the
    // controller may pay by sacrificing a land; declining sacrifices The
    // Gitrog Monster itself (the source, routed through DestroyPermanent).
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Land),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DestroyPermanent {
            target: trig.source,
        })),
    }]
}

fn draw_on_land_to_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}
