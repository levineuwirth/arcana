//! Vindictive Vampire — `{3}{B}` 2/3 black Vampire creature.
//! "Whenever another creature you control dies, this creature deals
//! 1 damage to each opponent and you gain 1 life." Models the
//! dies-trigger via a `ZoneChange` from battlefield to graveyard,
//! filtered to creatures you control, with the "another" clause
//! enforced inside the effect fn by comparing the dying object to
//! `trig.source`.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vindictive Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter::creature()
                    .controlled_by(ControllerConstraint::You),
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: on_another_creature_dies,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// On another creature you control dying: ping each opponent for 1
/// and gain 1 life. "Another" is enforced here by bailing if the
/// dying object is this creature itself.
fn on_another_creature_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    if let Some(dying) = trig.dying_object() {
        if dying == trig.source {
            return Vec::new();
        }
    }
    let mut out: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        out.push(Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 1,
            source: trig.source,
        });
    }
    out.push(Effect::GainLife {
        player: trig.controller,
        amount: 1,
    });
    out
}
