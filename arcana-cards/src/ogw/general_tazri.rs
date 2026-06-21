//! General Tazri — `{4}{W}` 3/4 legendary Human Ally.
//! "When General Tazri enters, you may search your library for an Ally
//! creature card, reveal it, put it into your hand, then shuffle.
//! {W}{U}{B}{R}{G}: Ally creatures you control get +X/+X until end of
//! turn, where X is the number of colors among those creatures."
//!
//! Abilities:
//! 1. ETB → search your library for an Ally creature card to hand.
//! 2. {W}{U}{B}{R}{G}: Ally creatures get +X/+X (X = colors among them)
//!    — the dynamic count-of-colors-among-a-set scaling has no script
//!    helper, so the pump amount can't be computed; GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("General Tazri");
    let human = reg.interner_mut().intern("Human");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: search_for_ally,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}{U}{B}{R}{G}: Ally creatures you control get +X/+X until end \
                       of turn, where X is the number of colors among those creatures."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}{U}{B}{R}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_allies,
            }),
    )
}

fn search_for_ally(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "search your library for an Ally creature card ... into your hand"
    let filter = script::subtype_filter(reg, "Ally").with_types(TypeLine::CREATURE.into());
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}

fn pump_allies(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "+X/+X where X is the number of colors among those creatures"
    // — there is no script helper for counting distinct colors among a
    // set of permanents, so the dynamic pump amount can't be computed.
    // Emitting a fixed +N/+N would be materially wrong.
    Vec::new()
}
