//! Ashling, the Limitless — `{2}{R}` 2/3 legendary red Elemental Sorcerer.
//!
//! * "Elemental permanent spells you cast from your hand gain evoke {4} as
//!   you cast them." — GAP: a static cost-granting ability (grants the Evoke
//!   alternative cost to other spells); not expressible as a triggered or
//!   activated ability, and there is no effect to grant Evoke.
//! * "Whenever you sacrifice a nontoken Elemental, create a token that's a
//!   copy of it. The token gains haste ...; at the beginning of your next
//!   end step, sacrifice it unless you pay {W}{U}{B}{R}{G}." — the
//!   Sacrificed trigger is faithful, but the effect is GAP'd: `CopyPermanent`
//!   copies a permanent on the battlefield, while the sacrificed Elemental
//!   has already left play (no live object to copy, and no accessor exposes
//!   the sacrificed object's id), so the copy-and-rider chain can't be
//!   built.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ashling, the Limitless");
    let elemental = reg.interner_mut().intern("Elemental");
    let sorcerer = reg.interner_mut().intern("Sorcerer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(sorcerer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::Sacrificed {
                filter: script::subtype_filter(reg, "Elemental").nontoken(),
            },
            intervening_if: None,
            effect: on_sacrifice_elemental,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_sacrifice_elemental(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: see module doc — cannot create a copy token of a permanent that
    // has already been sacrificed, so the haste / pay-or-sacrifice riders
    // are unreachable too.
    Vec::new()
}
