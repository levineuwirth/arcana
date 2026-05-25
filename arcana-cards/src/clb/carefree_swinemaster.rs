//! Carefree Swinemaster — `{2}{G}` 1/4 green Gnome Ranger.
//! "Whenever this creature attacks, you may pay {1}{G}. If you do, create a
//! 2/2 green Boar creature token that's tapped and attacking."
//! GAP: effect — conditional on paying mana is not in the Effect catalog;
//! "tapped and attacking" token state not expressible (CreateToken doesn't
//! set attacking). Emitting the token creation unconditionally as
//! best-effort.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Carefree Swinemaster");
    let gnome = reg.interner_mut().intern("Gnome");
    let ranger = reg.interner_mut().intern("Ranger");
    let _boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    subtypes.0.insert(ranger);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: on_attacks,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_attacks(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let boar = reg.interner().lookup("Boar").expect("Boar interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(boar);
    let token = TokenDefinition {
        name: boar,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: effect — conditional on paying {1}{G}; "tapped and attacking" state
    // not expressible with CreateToken.
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
