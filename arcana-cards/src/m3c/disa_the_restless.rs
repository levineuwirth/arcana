//! Disa the Restless — `{2}{B}{R}{G}` 5/6 Legendary Human Scout.
//!
//! * "Whenever a Lhurgoyf permanent card is put into your graveyard
//!   from anywhere other than the battlefield, put it onto the
//!   battlefield." — ZoneChange into the graveyard filtered to a
//!   Lhurgoyf you'd own; the triggering card is reanimated.
//! * "Whenever one or more creatures you control deal combat damage to
//!   a player, create a Tarmogoyf token." — combat-damage-to-player
//!   trigger minting a green Lhurgoyf token.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Disa the Restless");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let _lhurgoyf = reg.interner_mut().intern("Lhurgoyf");
    let _token_name = reg.interner_mut().intern("Tarmogoyf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    let lhurgoyf_filter = script::subtype_filter(reg, "Lhurgoyf")
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: "from anywhere other than the battlefield" — ZoneChange has no
            // "from anywhere except X" form; this over-fires on a Lhurgoyf that
            // dies from the battlefield as well.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: lhurgoyf_filter,
                    from: None,
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: reanimate_lhurgoyf,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: make_tarmogoyf,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn reanimate_lhurgoyf(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: id }]
}

fn make_tarmogoyf(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let lhurgoyf = reg.interner().lookup("Lhurgoyf").unwrap_or_default();
    let token_name = reg.interner().lookup("Tarmogoyf").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lhurgoyf);
    // GAP: a real Tarmogoyf token has the characteristic-defining */1+* power
    // and toughness keyed off graveyard card types; that CDA is not
    // expressible, so the token uses a fixed best-effort 0/1 body.
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
