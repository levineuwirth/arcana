//! The Miniaturizer — `{4}{W}` 5/5 Legendary Human Artificer.
//! "When The Miniaturizer enters, soospect up to one target creature."
//! "{4}{W}, Exile a creature card from your graveyard: Create a token that's
//!  a copy of the exiled card, except it's an Equipment artifact token …"
//!
//! The ETB "soospect" (a suspect-style flag) is wired via Effect::Suspect on
//! the chosen creature (up to one target). The activated equipment-copy
//! ability is a bespoke token-transform with custom riders (equip cost = mana
//! value, dynamic +X/+X) that has no faithful Effect form — its body is GAP'd,
//! though the activation cost (mana + exile a creature from graveyard) cannot
//! be expressed either (no exile-from-graveyard cost field), so the whole
//! activated ability is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Miniaturizer");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: activated ability — "{4}{W}, Exile a creature card from your
    // graveyard: create an Equipment copy token with dynamic +X/+X" cannot be
    // expressed (no exile-from-graveyard activation cost; bespoke token).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_suspect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature()),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_suspect(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Suspect { target: *id }]
}
