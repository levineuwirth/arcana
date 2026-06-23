//! Diminished Returner — `{1}{B}{B}` 4/3 Creature — Phyrexian Skeleton.
//!
//! * Diminished Returner enters tapped. — GAP: no enters-tapped primitive in
//!   this surface (a static replacement on the ETB).
//! * {B}{B}: Diminished Returner perpetually gets -1/-1, then return it to the
//!   battlefield. Activate only if Diminished Returner is in your graveyard and
//!   its toughness is 2 or greater. — a graveyard-activated ability
//!   (`ActivationZone::Graveyard`) gated by an activation condition (toughness
//!   ≥ 2). The "return it to the battlefield" half is wired with
//!   `ReturnFromGraveyardToBattlefield`.
//!   GAP: "perpetually gets -1/-1" — perpetual (Alchemy) modifications have no
//!   primitive here; the return half is wired faithfully.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Diminished Returner");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(skeleton);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "Diminished Returner enters tapped" — no enters-tapped primitive.

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}{B}: Diminished Returner perpetually gets -1/-1, then return it to the battlefield. Activate only if Diminished Returner is in your graveyard and its toughness is 2 or greater.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}{B}").expect("valid cost"),
                    activation_condition: Some(if_toughness_at_least_two),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self,
            }),
    )
}

/// "Activate only if ... its toughness is 2 or greater."
fn if_toughness_at_least_two(
    state: &GameState,
    source: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::toughness_of(state, source) >= 2
}

/// "return it to the battlefield." (perpetual -1/-1 is GAP'd)
fn return_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
