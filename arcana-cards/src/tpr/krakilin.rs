//! Krakilin — `{X}{G}{G}` 0/0 green Creature — Beast.
//! "This creature enters with X +1/+1 counters on it."
//! "{1}{G}: Regenerate this creature."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Krakilin");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    // GAP: "enters with X +1/+1 counters" — X (the cast cost) isn't readable
    // from a SelfEntersBattlefield trigger fn (no x_value on PendingTrigger),
    // and EntersWithSpec is not part of the demonstrated MultiAbilityCreature
    // API; the counters-on-enter clause is omitted.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{G}: Regenerate this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: regenerate_self,
        }),
    )
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
