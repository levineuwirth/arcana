//! Zaxara, the Exemplary — `{1}{B}{G}{U}` 2/3 Legendary Creature — Nightmare Hydra.
//! Deathtouch.
//! {T}: Add two mana of any one color.
//! Whenever you cast a spell with {X} in its mana cost, create a 0/0 green
//! Hydra creature token, then put X +1/+1 counters on it.
//!
//! GAP (mana ability): "Add two mana of any one color" needs a runtime
//! color choice; `Effect::AddMana` takes a fixed `ManaColor` per pip with
//! no any-color choice primitive — the activated ability's body is GAP'd
//! (same posture as Oasis Ritualist).
//! Partial (trigger): the spell-cast trigger creates the 0/0 green Hydra
//! token. GAP: there is no filter for "spell with {X} in its mana cost"
//! (it fires on every spell you cast — a fidelity over-fire), and no
//! accessor for the cast spell's X to place "X +1/+1 counters" — that half
//! is omitted.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zaxara, the Exemplary");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}{U}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add two mana of any one color.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_any_one_color,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_hydra_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_any_one_color(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "two mana of any ONE color" requires a runtime color choice;
    // Effect::AddMana takes a fixed ManaColor per pip with no choice form.
    Vec::new()
}

fn make_hydra_token(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: cannot read the cast spell's X to place "X +1/+1 counters";
    // only the 0/0 green Hydra token is created.
    let Some(hydra) = reg.interner().lookup("Hydra") else {
        return Vec::new();
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    let token = TokenDefinition {
        name: hydra,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}
