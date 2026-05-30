//! Thundering Wurm — `{2}{G}` 4/4 green Wurm.
//! "When this creature enters, sacrifice it unless you discard a land card."
//! GAP: OptionalPaymentKind only supports Mana and Life costs; discarding a
//! typed card ("a land card") as a payment gate is not expressible.
//! Emitting the unconditional sacrifice (the "unless you discard" guard is GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thundering Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_sacrifice_unless_discard_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_sacrifice_unless_discard_land(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless you discard a land card" — OptionalPaymentKind does not
    // support a discard-typed-card payment gate; the conditional is not
    // expressible. Emitting the unconditional self-sacrifice.
    vec![Effect::Sacrifice {
        player: trig.controller,
        filter: ObjectFilter::creature(),
        count: 1,
    }]
}
