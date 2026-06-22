//! Woe Strider — `{2}{B}` 3/2 Horror.
//! "When this creature enters, create a 0/1 white Goat creature token."
//! "Sacrifice another creature: Scry 1."
//! Escape—{3}{B}{B}, Exile four other cards from your graveyard.
//!
//! Decomposition:
//! * ETB Goat token — `SelfEntersBattlefield` trigger creating one 0/1 white
//!   Goat creature token.
//! * Sac-another-creature activation — `sacrifice_other` cost + Scry 1 effect.
//! * Escape — alternative graveyard-cast keyword not in supported surface;
//!   GAP'd (including "escapes with two +1/+1 counters").

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Woe Strider");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    // Pre-intern the Goat token subtype.
    let _goat = reg.interner_mut().intern("Goat");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: Escape—{3}{B}{B}, exile four other cards from graveyard (+ "escapes
    //      with two +1/+1 counters") — graveyard alternative-cast keyword not in
    //      supported surface.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_goat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another creature: Scry 1.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sac_scry,
            }),
    )
}

fn etb_make_goat(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let goat = reg.interner().lookup("Goat").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goat);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: goat,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn sac_scry(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry {
        player: ctx.controller,
        count: 1,
    }]
}
