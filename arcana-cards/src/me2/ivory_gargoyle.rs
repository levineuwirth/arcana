//! Ivory Gargoyle — `{4}{W}` 2/2 white Gargoyle with Flying.
//!
//! * Flying.
//! * When this creature dies, return it to the battlefield under its owner's
//!   control at the beginning of the next end step and you skip your next draw
//!   step.
//! * `{4}{W}: Exile this creature.`
//!
//! The dies trigger is GAP'd: returning a creature from the graveyard to the
//! battlefield at a delayed moment has no `DelayedAction` variant (the set is
//! Sacrifice / Exile / ReturnToHand / ReturnFromExileToBattlefield /
//! ControllerDrawsCard — none re-enters from the graveyard), and "skip your
//! next draw step" has no expressible primitive. Flying and the self-exile
//! activated ability are expressed faithfully.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ivory Gargoyle");
    let gargoyle = reg.interner_mut().intern("Gargoyle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gargoyle);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return_and_skip_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Exile this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_self,
            }),
    )
}

fn dies_return_and_skip_draw(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return it to the battlefield ... at the beginning of the next end
    // step" needs a delayed graveyard->battlefield return, which no
    // DelayedAction variant provides; "skip your next draw step" likewise has
    // no expressible primitive.
    Vec::new()
}

fn exile_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ExilePermanent { target: ctx.source }]
}
