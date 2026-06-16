//! Undercellar Myconid — `{2}{G}` 1/2 Fungus.
//!
//! Oracle:
//! * "Whenever this creature enters or dies, create a 1/1 green Saproling
//!   creature token." — decomposed into two triggered abilities (one on
//!   `SelfEntersBattlefield`, one on `SelfDies`), each minting a Saproling.
//! * "{T}: Add one mana of any color." — a tap-only mana ability. The
//!   "any color" choice is not expressible with the demonstrated mana API
//!   (`ManaUnit::plain` requires a concrete `ManaColor`, and there is no
//!   any-color variant shown), so this ability is GAP'd.

use arcana_core::effects::Effect;
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
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undercellar Myconid");
    let fungus = reg.interner_mut().intern("Fungus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    // Pre-intern the token subtype so the resolvers can recover it.
    let _saproling = reg.interner_mut().intern("Saproling");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: make_saproling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add one mana of any color.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_color,
            }),
    )
}

/// "create a 1/1 green Saproling creature token."
fn make_saproling(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Saproling").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(name);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// "{T}: Add one mana of any color."
fn add_any_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "add one mana of any color" — the demonstrated mana API
    // (`ManaUnit::plain(color, source)`) requires a concrete `ManaColor`
    // and no any-color / choose-a-color mana variant is shown.
    Vec::new()
}
