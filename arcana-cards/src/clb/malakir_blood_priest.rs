//! Malakir Blood-Priest — `{1}{B}` 2/1 Vampire Cleric. "When this
//! creature enters, each opponent loses X life and you gain X life,
//! where X is the number of creatures in your party. (Your party
//! consists of up to one each of Cleric, Rogue, Warrior, and
//! Wizard.)"
//!
//! Party is approximated as the number of the four party subtypes
//! for which the controller has at least one creature on the
//! battlefield. This does not perfectly model the "up to one each"
//! assignment in CR 700.10 when creatures have multiple party
//! subtypes (e.g. a Cleric Wizard can only fill one slot), but it
//! matches the common case.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Malakir Blood-Priest");
    let vampire = reg.interner_mut().intern("Vampire");
    let cleric = reg.interner_mut().intern("Cleric");
    // Pre-intern the other party subtypes so the resolver can look
    // them up via the non-mut interner at trigger resolution time.
    let _rogue = reg.interner_mut().intern("Rogue");
    let _warrior = reg.interner_mut().intern("Warrior");
    let _wizard = reg.interner_mut().intern("Wizard");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_party_drain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: count the party subtypes the controller has at
/// least one creature of, then each opponent loses X life and the
/// controller gains X life.
fn etb_party_drain(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut x: u32 = 0;
    for ty in ["Cleric", "Rogue", "Warrior", "Wizard"].iter() {
        let filter = script::subtype_filter(reg, ty)
            .controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &filter, trig.controller) >= 1 {
            x += 1;
        }
    }

    let mut effects: Vec<Effect> = Vec::new();
    for opp in script::opponents(state, trig.controller) {
        effects.push(Effect::LoseLife { player: opp, amount: x });
    }
    effects.push(Effect::GainLife { player: trig.controller, amount: x });
    effects
}
