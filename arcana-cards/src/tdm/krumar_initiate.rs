//! Krumar Initiate — `{1}{B}` 2/2 Creature — Human Cleric.
//! `{X}{B}, {T}, Pay X life: This creature endures X.`
//! (Endure X: Put X +1/+1 counters on it or create an X/X white Spirit creature token.)
//! GAP: "Pay X life" — the life cost is dynamic (equals the X paid), but
//!      ActivationCost.life is a fixed value with no coupling to x_value.
//! GAP: "endures X" — player choice between X +1/+1 counters or an X/X Spirit
//!      token is not expressible (no choose-mode-then-X effect).
//! (The generic-{X} mana cost itself now fans out; both remaining blockers are
//!  the X-coupled life cost and the Endure choice, so the ability is omitted.)

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
    let name = reg.interner_mut().intern("Krumar Initiate");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{X}{B}, {T}, Pay X life: This creature endures X. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                // GAP: {X} variable cost and "Pay X life" dynamic not in ActivationCost
                // GAP: Endure mechanic (counter or token choice) not expressible
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: endure,
            }),
    )
}

fn endure(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: X variable and Endure choice not expressible
    Vec::new()
}
