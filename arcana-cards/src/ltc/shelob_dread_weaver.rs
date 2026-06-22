//! Shelob, Dread Weaver — `{3}{B}` 3/3 Legendary Spider Demon.
//!
//! * Whenever a nontoken creature an opponent controls dies, exile it.
//! * `{2}{B}, Put a creature card exiled with Shelob into its owner's
//!   graveyard: Put two +1/+1 counters on Shelob. Draw a card.` — GAP: the cost
//!   "put a card exiled with this into its owner's graveyard" is not an
//!   expressible ActivationCost field, and there is no exiled-with-source link.
//! * `{X}{1}{B}: Put target creature card with mana value X exiled with Shelob
//!   onto the battlefield tapped under your control.` — GAP: no way to target /
//!   reanimate from the "exiled with this source" set.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shelob, Dread Weaver");
    let spider = reg.interner_mut().intern("Spider");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);
    subtypes.0.insert(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // Whenever a nontoken creature an opponent controls dies, exile it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: exile_dying_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: activated ability — "{2}{B}, Put a creature card exiled with
        // Shelob into its owner's graveyard: Put two +1/+1 counters on Shelob.
        // Draw a card." The exile-into-graveyard cost over the
        // exiled-with-this set is not an expressible ActivationCost field.
        // GAP: activated ability — "{X}{1}{B}: Put target creature card with
        // mana value X exiled with Shelob onto the battlefield tapped under
        // your control." No targeting over the exiled-with-this set.
    )
}

fn exile_dying_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(id) = trig.dying_object() else { return Vec::new(); };
    vec![Effect::ExilePermanent { target: id }]
}
