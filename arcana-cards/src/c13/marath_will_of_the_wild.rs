//! Marath, Will of the Wild — `{R}{G}{W}` Legendary 0/0 Elemental Beast.
//!
//! Rules text:
//! * Marath enters with a number of +1/+1 counters on it equal to the amount of
//!   mana spent to cast it. (static — GAP, no cast-mana hook)
//! * {X}, Remove X +1/+1 counters from Marath: Choose one —
//!     • Put X +1/+1 counters on target creature. X can't be 0.
//!     • Marath deals X damage to any target. X can't be 0.
//!     • Create an X/X green Elemental creature token. X can't be 0.
//!
//! Both abilities are GAP'd. "Enters with counters equal to mana spent" has no
//! demonstrated cast-time hook. The activated ability is a MODAL "choose one"
//! with a variable X cost ({X}, Remove X +1/+1 counters); modal mode-selection
//! is only available on SPELL abilities, and the X-scaled cost/effect has no
//! demonstrated representation. The activated ability is recorded with its
//! oracle text and a placeholder remove-counter cost; the modal payload is a
//! no-op GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marath, Will of the Wild");
    let elemental = reg.interner_mut().intern("Elemental");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP (static): "enters with +1/+1 counters equal to mana spent to cast it"
        //       — no demonstrated cast-time mana hook.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}, Remove X +1/+1 counters from Marath: Choose one — Put X +1/+1 counters on target creature; Marath deals X damage to any target; Create an X/X green Elemental creature token.".into(),
            cost: ActivationCost {
                // GAP: the {X} mana and "Remove X +1/+1 counters" variable cost is
                //      not expressible; recorded as a single-counter removal placeholder.
                remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: marath_modal,
        }),
    )
}

fn marath_modal(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal "choose one" on an activated ability is not expressible (modal
    //       dispatch is spell-ability-only), and the X-scaled payload depends on
    //       the X paid for the variable cost, which has no hook.
    Vec::new()
}
