//! Koma and Toski, Compleated — `{6}{U}{G}` 7/7 Legendary Phyrexian Serpent
//! Squirrel.
//! "This spell can't be countered."
//! "At the beginning of each upkeep, create a 1/1 Phyrexian Serpent Squirrel
//! artifact creature token named Toski's Coil with 'Whenever this creature
//! deals combat damage to a player, draw a card.'"
//! "Sacrifice another Serpent or Squirrel: Choose one — Target creature attacks
//! this turn if able; its activated abilities can't be activated this turn. /
//! Koma and Toski, Compleated gains indestructible until end of turn."
//!
//! "Can't be countered" is a cast-time static — GAP'd. The each-upkeep trigger
//! is wired and mints the token; the token's embedded "this creature deals
//! combat damage → draw" ability has no self-referential DamageDealt source in
//! this class (a broad source_filter would over-fire), so the token is minted
//! vanilla with that ability GAP'd. The sacrifice activated ability's cost is
//! wired (sacrifice another Serpent or Squirrel), but its modal "choose one"
//! payload is not expressible on an activated ability — GAP'd.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Koma and Toski, Compleated");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let serpent = reg.interner_mut().intern("Serpent");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(serpent);
    subtypes.0.insert(squirrel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![],
        ..Default::default()
    };

    // Build the sacrifice-cost filter from already-interned symbols before the
    // `reg.register` borrow (avoids a double mutable borrow of `reg`).
    let sac_filter = ObjectFilter::creature().with_subtypes_any(vec![serpent, squirrel]);

    // GAP: "This spell can't be countered" — cast-time static, no primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: make_coil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another Serpent or Squirrel: Choose one — \
                       Target creature attacks this turn if able; its activated \
                       abilities can't be activated this turn. / Koma and Toski, \
                       Compleated gains indestructible until end of turn."
                    .into(),
                cost: ActivationCost {
                    sacrifice_other: Some(sac_filter),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: sac_choose_one,
            }),
    )
}

fn make_coil(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let phyrexian = reg.interner().lookup("Phyrexian").unwrap_or_default();
    let serpent = reg.interner().lookup("Serpent").unwrap_or_default();
    let squirrel = reg.interner().lookup("Squirrel").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(serpent);
    subtypes.0.insert(squirrel);
    // GAP: token's embedded "Whenever this creature deals combat damage to a
    // player, draw a card" — no self-referential DamageDealt source is
    // expressible (a broad source_filter would over-fire); token is minted
    // without the ability.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: serpent,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn sac_choose_one(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: modal "choose one" payload is not expressible on an activated
    // ability (modal selection is spell-ability only).
    Vec::new()
}
